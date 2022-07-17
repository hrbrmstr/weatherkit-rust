// #[allow(unused_imports)]
// use log::{info, trace, warn};

use std::time::{UNIX_EPOCH, SystemTime};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::collections::HashSet;

use serde_json::json;

use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID, encode};

use tz_search::{lookup};

use format_num::NumberFormat;

use pad::{PadStr, Alignment};

use chrono::prelude::*;
use chrono::{DateTime, Utc, SecondsFormat};
use chrono_tz::Tz;

extern crate colored;
use colored::{*};

use clap::{Parser};

mod weatherkitweather;
use weatherkitweather::{*};

mod placename;
use placename::{*};

mod utils;
use utils::{*};

/// Mandatory Apple Weather trademark. The Apple glyph is a pain
/// to type so this makes it a bit easier to handle
const APPLE_WEATHER_TRADEMARK: &'static str = " Weather";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

  // latitude
  #[clap(long, default_value_t = String::from("43.2683199"))]
  lat: String,

  // longitude
  #[clap(long, default_value_t = String::from("-70.8635506"))]
  lon: String,

  // language
  #[clap(long, default_value_t = String::from("en"))]
  lang: String
  
}

/// Setup the JSON Web Token from the required environment variables.
/// 
/// # Arguments
/// 
/// * None
/// 
/// # Required Environment Variables
/// 
/// - `WEATHERKIT_KEY_ID`
/// - `WEATHERKIT_SERVICE_ID`
/// - `WEATHERKIT_TEAM_ID`
/// - `WEATHERKIT_KEY_PATH`
/// 
/// See <https://github.com/hrbrmstr/weatherkit-rust/blob/batman/authorization.md> for how to 
/// set up those values.
fn setup_jwt() -> String {
  
  let wxkit_keyid = env::var("WEATHERKIT_KEY_ID").expect("Please set WEATHERKIT_KEY_ID");
  let wxkit_service_id = env::var("WEATHERKIT_SERVICE_ID").expect("Please set WEATHERKIT_SERVICE_ID");
  let wxkit_teamid = env::var("WEATHERKIT_TEAM_ID").expect("Please set WEATHERKIT_TEAM_ID");
  let wxkit_key_path = env::var("WEATHERKIT_KEY_PATH").expect("Please set WEATHERKIT_KEY_PATH");
  
  let f = File::open(wxkit_key_path.as_str()).expect("Error opening key file at WEATHERKIT_KEY_PATH");
  let mut reader = BufReader::new(f);
  let mut buffer = Vec::new();    
  reader.read_to_end(&mut buffer).expect("Error reading key file at WEATHERKIT_KEY_PATH");
  let key_buf: &[u8] = &buffer;
  
  let alg = Algorithm::new_ecdsa_pem_signer(AlgorithmID::ES256, key_buf).expect("Error decoding PEM");
  
  let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Timey wimey").as_secs();
  let expires = now + 3600;
  
  let claims = json!({
    "iss": wxkit_teamid.to_owned(),
    "iat": now,
    "exp": expires,
    "sub": wxkit_service_id.to_owned()
  });
  
  let header = json!({
    "alg": "ES256",
    "kid": wxkit_keyid.to_owned(),
    "id": format!("{}.{}", wxkit_teamid.to_owned(), wxkit_service_id.to_owned())
  });
  
  return encode(&header, &claims, &alg).expect("Error creating JWT");

}

/// Retrieve weather data from Apple's WeatherKit REST API
/// 
/// # Arguments
/// 
/// - `token` - JWT from [setup_jwt()] call
/// - `latitude` - latitude
/// - `longitude` - longitude
/// - `language` - language code
/// - `tzone` - time zone string
/// - `utc_now` - "now" in UTC
fn get_weatherkit_weather(token: String, 
                          latitude: String,
                          longitude: String, 
                          language: String, 
                          tzone: String, 
                          utc_now: DateTime<Utc>) -> 
                          weatherkitweather::WeatherKitWeather {
  
  let utc_now_fmt = utc_now.to_rfc3339_opts(SecondsFormat::Secs, true);

  let url = format!(
    "https://weatherkit.apple.com/api/v1/weather/{}/{}/{}?timezone={}&dataSets=currentWeather,forecastDaily,forecastHourly,weatherAlerts&dailyStart={}&hourlyStart={}", 
    language, latitude, longitude, tzone, utc_now_fmt, utc_now_fmt
  );
  
  let client = reqwest::blocking::Client::new();
  
  let resp = client
    .get(url)
    .header("Authorization", format!("Bearer {}", token))
    .header("Accept", "application/json")
    .send().expect("Error retrieving weather from WeatherKit REST API")
    .json::<WeatherKitWeather>()
    .expect("Error retrieving weather info.");
  
  return resp;

}

/// Print everything
/// 
/// Setup the JWT, parse the cmdline args, estimate timezone, get the weather, print all the things.
fn main() {

  // simple_logger::SimpleLogger::new().env().init().unwrap();

  let token = setup_jwt();

  let args = Args::parse();

  let lat = args.lat;
  let lon = args.lon;
  
  let place = get_placename(lat.parse().unwrap(), lon.parse().unwrap());
  
  let tzone = lookup(lat.parse().unwrap(), lon.parse().unwrap()).unwrap();
  let tz: Tz = tzone.parse().unwrap();

  let utc_now = Utc::now();

  let resp = get_weatherkit_weather(token, lat.to_owned(), lon.to_owned(), args.lang, tzone, utc_now);
  
  let num = NumberFormat::new();

  let as_of_time = DateTime::parse_from_rfc3339(resp.current_weather.as_of.as_str()).unwrap().with_timezone(&tz);
  
  // Attribution labeling required by Apple
  // println!("{} daily forecast for ({}, {}) as of {}\n", APPLE_WEATHER_TRADEMARK, resp.current_weather.metadata.latitude, resp.current_weather.metadata.longitude, as_of_time);
  println!("{} daily forecast for {} as of {}\n", APPLE_WEATHER_TRADEMARK, place, as_of_time);
  
  // TODO metric option for everything that's displayed

  println!(" Conditions: {}",         format!("{}", condition_code(&resp.current_weather.condition_code)));
  println!("Temperature: {}°F",       num.format(".1f", c_to_f(resp.current_weather.temperature)));
  println!(" Feels like: {}°F",       num.format(".1f", c_to_f(resp.current_weather.temperature_apparent)));
  println!("  Dew Point: {}°F",       num.format(".1f", c_to_f(resp.current_weather.temperature_dew_point)));
  println!("       Wind: {} mph",     num.format(".0f", kmph_to_mph(resp.current_weather.wind_speed)));
  println!("   Pressure: {} mb ({})", num.format(".0f", resp.current_weather.pressure), format!("{:?}", resp.current_weather.pressure_trend));
  println!(" Visibility: {} miles",   meters_to_miles(resp.current_weather.visibility) as i64);
  println!("   UV Index: {} {}",      uv_label(resp.current_weather.uv_index, true), uv_label(resp.current_weather.uv_index, false));
  println!();

  let hrs = resp.forecast_hourly.hours;
  
  // get max lengths of various values so we can pad properly and not too much

  let max_hr_temp_len = hrs[0..23].iter().fold(0, |accum, hr| {
    let hr_len = num.format(".0f", c_to_f(hr.temperature)).len();
    if accum >= hr_len { accum } else { hr_len }
  });
  
  let max_hr_humid_len = hrs[0..23].iter().fold(0, |accum, hr| {
    let hr_len = num.format(".0f", hr.humidity * 100.0).len();
    if accum >= hr_len { accum } else { hr_len }
  });

  let max_hr_condiiton_len = hrs[0..23].iter().fold(0, |accum, hr| {
    let hr_len = condition_code(&hr.condition_code).len();
    if accum >= hr_len { accum } else { hr_len }
  });
   
  // keep track of day's we've seen & printed so we don't print them more than once
  let mut day_set: HashSet<String> = HashSet::new();

  for idx in 0..hrs[0..23].len() {

    let hour = &hrs[idx];
  
    let utc_fcast_time = DateTime::parse_from_rfc3339(hour.forecast_start.as_str()).unwrap();
    let local_time = utc_fcast_time.with_timezone(&tz);
    let weekday = format!("{}", local_time.weekday()).pad_to_width_with_alignment(5, Alignment::Right);
    let printed_str = if local_time.date() == utc_now.date() { "Today" } else { weekday.as_str() };
    
        println!(
      "{} @ {}:00 │ 🌡  {}°F │ 💦 {}% │ {} mb {} │ {} │ {} │ {}",
      if day_set.contains(printed_str) { "     " } else { printed_str },
      num.format("02d", local_time.hour()),
      num.format(".0f", c_to_f(hour.temperature)).pad_to_width_with_alignment(max_hr_temp_len, Alignment::Right),
      num.format(".0f", hour.humidity * 100.0).pad_to_width_with_alignment(max_hr_humid_len, Alignment::Right),
      num.format("4.0f", hour.pressure),
      match &hour.pressure_trend {
        Some(trend) => pressure_trend(trend),
        None => " ".to_string()
      },
      precip_type(&hour.precipitation_type, match hour.daylight { Some(dl) => dl, None => true }),
      condition_code(&hour.condition_code).pad_to_width(max_hr_condiiton_len),
      uv_label(hour.uv_index, true)
    );

    day_set.insert(printed_str.to_string());
    
  }
  
  println!();

  let days = resp.forecast_daily.days;

  // get all the info we need to rescale the temp ranges to 1..30 (# chars in the fake bars)

  let min_temps: Vec<i64> = days.iter().map(|d| c_to_f(d.temperature_min) as i64).collect();
  let max_temps: Vec<i64> = days.iter().map(|d| c_to_f(d.temperature_max) as i64).collect();

  let min_temp = min_temps.iter().min().expect("INCOSPIGLE!");
  let max_temp = max_temps.iter().max().expect("INCOSPIGLE!");

  let max_day_temp_len = days.iter().fold(0, |accum, day| {
    let day_len = num.format("d", c_to_f(day.temperature_max) as i32).len();
    if accum >= day_len { accum } else { day_len }
  });

  let block = "▆";

  for idx in 0..days.len() {

    let day = &days[idx];

    let utc_fcast_time = DateTime::parse_from_rfc3339(day.forecast_start.as_str()).unwrap();
    let local_time = utc_fcast_time.with_timezone(&tz);
    let weekday = format!("{}", local_time.weekday()).pad_to_width_with_alignment(5, Alignment::Right);
    let weekday_str = if local_time.date() == utc_now.date() { "Today" } else { weekday.as_str() };

    let day_min = c_to_f(day.temperature_min) as i32; // num.format doesn't like i64
    let day_max = c_to_f(day.temperature_max) as i32;
    let day_min_scaled = rescale_val(day_min as i64, *min_temp, *max_temp, 1, 30);
    let day_max_scaled = rescale_val(day_max as i64, *min_temp, *max_temp, 1, 30);

    print!(
      "{} {}°F ", 
      weekday_str,
      num.format("d", day_min).pad_to_width_with_alignment(max_day_temp_len, Alignment::Right),
    );

    for cell in 1..30 {
      if cell >= day_min_scaled && cell <= day_max_scaled {
        print!("{}", block.color("yellow"));
      } else {
        print!("{}", block.color("bright black"));
      }
    }
    
    println!(
      " {}°F {} {}",
      num.format("d", day_max).pad_to_width_with_alignment(max_day_temp_len, Alignment::Left),
      uv_label(day.max_uv_index, true),
      condition_code(&day.condition_code)
    );

  }

  println!();

  println!("{}", resp.current_weather.metadata.attribution_url); // Attribution labeling required by Apple
  
}
