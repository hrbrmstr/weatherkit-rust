use std::time::{UNIX_EPOCH, SystemTime};
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serde_json::json;

use jsonwebtokens as jwt;
use jwt::{Algorithm, AlgorithmID, encode};

use tz_search::{lookup};

use format_num::NumberFormat;

use pad::{PadStr, Alignment};

use chrono::prelude::*;
use chrono::{DateTime, Utc, SecondsFormat};
use chrono_tz::Tz;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherKitWeather {
  #[serde(rename = "currentWeather")]
  pub current_weather: CurrentWeather,
  
  #[serde(rename = "forecastDaily")]
  pub forecast_daily: ForecastDaily,
  
  #[serde(rename = "forecastHourly")]
  pub forecast_hourly: ForecastHourly,
  
  #[serde(rename = "forecastNextHour")]
  pub forecast_next_hour: Option<ForecastNextHour>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
  #[serde(rename = "name")]
  pub name: String,
  
  #[serde(rename = "metadata")]
  pub metadata: Metadata,
  
  #[serde(rename = "asOf")]
  pub as_of: String,
  
  #[serde(rename = "cloudCover")]
  pub cloud_cover: f64,
  
  #[serde(rename = "conditionCode")]
  pub condition_code: ConditionCode,
  
  #[serde(rename = "daylight")]
  pub daylight: bool,
  
  #[serde(rename = "humidity")]
  pub humidity: f64,
  
  #[serde(rename = "precipitationIntensity")]
  pub precipitation_intensity: f64,
  
  #[serde(rename = "pressure")]
  pub pressure: f64,
  
  #[serde(rename = "pressureTrend")]
  pub pressure_trend: PressureTrend,
  
  #[serde(rename = "temperature")]
  pub temperature: f64,
  
  #[serde(rename = "temperatureApparent")]
  pub temperature_apparent: f64,
  
  #[serde(rename = "temperatureDewPoint")]
  pub temperature_dew_point: f64,
  
  #[serde(rename = "uvIndex")]
  pub uv_index: f64,
  
  #[serde(rename = "visibility")]
  pub visibility: f64,
  
  #[serde(rename = "windDirection")]
  pub wind_direction: f64,
  
  #[serde(rename = "windGust")]
  pub wind_gust: f64,
  
  #[serde(rename = "windSpeed")]
  pub wind_speed: f64,
  
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HourWeatherconditions {
  
  #[serde(rename = "cloudCover")]
  pub cloud_cover: f64,
  
  #[serde(rename = "conditionCode")]
  pub condition_code: ConditionCode,
  
  #[serde(rename = "daylight")]
  pub daylight: bool,
  
  #[serde(rename = "humidity")]
  pub humidity: f64,
  
  #[serde(rename = "precipitationIntensity")]
  pub precipitation_intensity: f64,
  
  #[serde(rename = "pressure")]
  pub pressure: f64,
  
  #[serde(rename = "pressureTrend")]
  pub pressure_trend: PressureTrend,
  
  #[serde(rename = "temperature")]
  pub temperature: f64,
  
  #[serde(rename = "temperatureApparent")]
  pub temperature_apparent: f64,
  
  #[serde(rename = "temperatureDewPoint")]
  pub temperature_dew_point: f64,
  
  #[serde(rename = "uvIndex")]
  pub uv_index: f64,
  
  #[serde(rename = "visibility")]
  pub visibility: f64,
  
  #[serde(rename = "windDirection")]
  pub wind_direction: f64,
  
  #[serde(rename = "windGust")]
  pub wind_gust: f64,
  
  #[serde(rename = "windSpeed")]
  pub wind_speed: f64,
  
  #[serde(rename = "forecastStart")]
  pub forecast_start: String,
  
  #[serde(rename = "precipitationAmount")]
  pub precipitation_amount: f64,
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationType")]
  pub precipitation_type: PrecipitationType,
  
  #[serde(rename = "snowfallIntensity")]
  pub snowfall_intensity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  #[serde(rename = "attributionURL")]
  pub attribution_url: String,
  
  #[serde(rename = "expireTime")]
  pub expire_time: String,
  
  #[serde(rename = "latitude")]
  pub latitude: f64,
  
  #[serde(rename = "longitude")]
  pub longitude: f64,
  
  #[serde(rename = "readTime")]
  pub read_time: String,
  
  #[serde(rename = "reportedTime")]
  pub reported_time: Option<String>,
  
  #[serde(rename = "units")]
  pub units: String,
  
  #[serde(rename = "version")]
  pub version: f64,
  
  #[serde(rename = "language")]
  pub language: Option<String>,
  
  #[serde(rename = "providerName")]
  pub provider_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastDaily {
  #[serde(rename = "name")]
  pub name: String,
  
  #[serde(rename = "metadata")]
  pub metadata: Metadata,
  
  #[serde(rename = "days")]
  pub days: Vec<Day>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
  #[serde(rename = "forecastStart")]
  pub forecast_start: String,
  
  #[serde(rename = "forecastEnd")]
  pub forecast_end: String,
  
  #[serde(rename = "conditionCode")]
  pub condition_code: String,
  
  #[serde(rename = "maxUvIndex")]
  pub max_uv_index: f64,
  
  #[serde(rename = "moonPhase")]
  pub moon_phase: MoonPhase,
  
  #[serde(rename = "moonrise")]
  pub moonrise: String,
  
  #[serde(rename = "moonset")]
  pub moonset: String,
  
  #[serde(rename = "precipitationAmount")]
  pub precipitation_amount: f64,
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationType")]
  pub precipitation_type: PrecipitationType,
  
  #[serde(rename = "snowfallAmount")]
  pub snowfall_amount: f64,
  
  #[serde(rename = "solarMidnight")]
  pub solar_midnight: String,
  
  #[serde(rename = "solarNoon")]
  pub solar_noon: String,
  
  #[serde(rename = "sunrise")]
  pub sunrise: String,
  
  #[serde(rename = "sunriseCivil")]
  pub sunrise_civil: String,
  
  #[serde(rename = "sunriseNautical")]
  pub sunrise_nautical: String,
  
  #[serde(rename = "sunriseAstronomical")]
  pub sunrise_astronomical: String,
  
  #[serde(rename = "sunset")]
  pub sunset: String,
  
  #[serde(rename = "sunsetCivil")]
  pub sunset_civil: String,
  
  #[serde(rename = "sunsetNautical")]
  pub sunset_nautical: String,
  
  #[serde(rename = "sunsetAstronomical")]
  pub sunset_astronomical: String,
  
  #[serde(rename = "temperatureMax")]
  pub temperature_max: f64,
  
  #[serde(rename = "temperatureMin")]
  pub temperature_min: f64,
  
  #[serde(rename = "daytimeForecast")]
  pub daytime_forecast: Forecast,
  
  #[serde(rename = "overnightForecast")]
  pub overnight_forecast: Option<Forecast>,
  
  #[serde(rename = "restOfDayForecast")]
  pub rest_of_day_forecast: Option<Forecast>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
  #[serde(rename = "forecastStart")]
  pub forecast_start: String,
  
  #[serde(rename = "forecastEnd")]
  pub forecast_end: String,
  
  #[serde(rename = "cloudCover")]
  pub cloud_cover: f64,
  
  #[serde(rename = "conditionCode")]
  pub condition_code: ConditionCode,
  
  #[serde(rename = "humidity")]
  pub humidity: f64,
  
  #[serde(rename = "precipitationAmount")]
  pub precipitation_amount: f64,
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationType")]
  pub precipitation_type: PrecipitationType,
  
  #[serde(rename = "snowfallAmount")]
  pub snowfall_amount: f64,
  
  #[serde(rename = "windDirection")]
  pub wind_direction: f64,
  
  #[serde(rename = "windSpeed")]
  pub wind_speed: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastHourly {
  #[serde(rename = "name")]
  pub name: String,
  
  #[serde(rename = "metadata")]
  pub metadata: Metadata,
  
  #[serde(rename = "hours")]
  pub hours: Vec<HourWeatherconditions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastNextHour {
  #[serde(rename = "name")]
  pub name: String,
  
  #[serde(rename = "metadata")]
  pub metadata: Metadata,
  
  #[serde(rename = "summary")]
  pub summary: Vec<Minute>,
  
  #[serde(rename = "forecastStart")]
  pub forecast_start: String,
  
  #[serde(rename = "forecastEnd")]
  pub forecast_end: String,
  
  #[serde(rename = "minutes")]
  pub minutes: Vec<Minute>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minute {
  #[serde(rename = "startTime")]
  pub start_time: String,
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationIntensity")]
  pub precipitation_intensity: f64,
  
  #[serde(rename = "condition")]
  pub condition: Option<PrecipitationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConditionCode {
  #[serde(rename = "Clear")]
  Clear,
  
  #[serde(rename = "Cloudy")]
  Cloudy,
  
  #[serde(rename = "Dust")]
  Dust,
  
  #[serde(rename = "Fog")]
  Fog,
  
  #[serde(rename = "Haze")]
  Haze,
  
  #[serde(rename = "MostlyClear")]
  MostlyClear,
  
  #[serde(rename = "MostlyCloudy")]
  MostlyCloudy,
  
  #[serde(rename = "PartlyCloudy")]
  PartlyCloudy,
  
  #[serde(rename = "ScatteredThunderstorms")]
  ScatteredThunderstorms,
  
  #[serde(rename = "Smoke")]
  Smoke,
  
  #[serde(rename = "Breezy")]
  Breezy,
  
  #[serde(rename = "Windy")]
  Windy,
  
  #[serde(rename = "Drizzle")]
  Drizzle,
  
  #[serde(rename = "HeavyRain")]
  HeavyRain,
  
  #[serde(rename = "Rain")]
  Rain,
  
  #[serde(rename = "Showers")]
  Showers,
  
  #[serde(rename = "Flurries")]
  Flurries,
  
  #[serde(rename = "HeavySnow")]
  HeavySnow,
  
  #[serde(rename = "MixedRainAndSleet")]
  MixedRainAndSleet,
  
  #[serde(rename = "MixedRainAndSnow")]
  MixedRainAndSnow,
  
  #[serde(rename = "MixedRainfall")]
  MixedRainfall,
  
  #[serde(rename = "MixedSnowAndSleet")]
  MixedSnowAndSleet,
  
  #[serde(rename = "ScatteredShowers")]
  ScatteredShowers,
  
  #[serde(rename = "ScatteredSnowShowers")]
  ScatteredSnowShowers,
  
  #[serde(rename = "Sleet")]
  Sleet,
  
  #[serde(rename = "Snow")]
  Snow,
  
  #[serde(rename = "SnowShowers")]
  SnowShowers,
  
  #[serde(rename = "Blizzard")]
  Blizzard,
  
  #[serde(rename = "BlowingSnow")]
  BlowingSnow,
  
  #[serde(rename = "FreezingDrizzle")]
  FreezingDrizzle,
  
  #[serde(rename = "FreezingRain")]
  FreezingRain,
  
  #[serde(rename = "Frigid")]
  Frigid,
  
  #[serde(rename = "Hail")]
  Hail,
  
  #[serde(rename = "Hot")]
  Hot,
  
  #[serde(rename = "Hurricane")]
  Hurricane,
  
  #[serde(rename = "IsolatedThunderstorms")]
  IsolatedThunderstorms,
  
  #[serde(rename = "SevereThunderstorm")]
  SevereThunderstorm,
  
  #[serde(rename = "Thunderstorm")]
  Thunderstorm,
  
  #[serde(rename = "Tornado")]
  Tornado,
  
  #[serde(rename = "TropicalStorm")]
  TropicalStorm,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrecipitationType {
  #[serde(rename = "clear")]
  Clear,
  
  #[serde(rename = "rain")]
  Rain,
  
  #[serde(rename = "hail")]
  Hail,
  
  #[serde(rename = "mixed")]
  Mixed,
  
  #[serde(rename = "snow")]
  Snow,
  
  #[serde(rename = "sleet")]
  Sleet,
  
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PressureTrend {
  #[serde(rename = "falling")]
  Falling,
  
  #[serde(rename = "rising")]
  Rising,
  
  #[serde(rename = "steady")]
  Steady,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MoonPhase {
  #[serde(rename="firstQuarter")]
  FirstQuarter,
  
  #[serde(rename="full")]
  Full,
  
  #[serde(rename="lastQuarter")]
  LastQuarter,
  
  #[serde(rename="thirdQuarter")]
  ThirdQuarter,
  
  #[serde(rename="seconduarter")]
  SecondQuarter,
  
  #[serde(rename="new")]
  New,
  
  #[serde(rename="waningCrescent")]
  WaningCrescent,
  
  #[serde(rename="waningGibbous")]
  WaningGibbous,
  
  #[serde(rename="waxingCrescent")]
  WaxingCrescent,
  
  #[serde(rename="waxingGibbous")]
  WaxingGibbous,
}

const APPLE_WEATHER_TRADEMARK: &'static str = " Weather";

fn c_to_f(temp: f64) -> f64 {
  (9.0/5.0) * temp + 32.0
}

fn meters_to_miles(meters: f64) -> i64 {
  (meters * 0.000621) as i64
}

fn kmph_to_mph(kmph: f64) -> f64 {
  kmph * 0.539593 * 1.1507794
}

fn uv_label(uv_index: f64, swatch: bool) -> String {
  
  String::from(match uv_index as i64 {
    0 | 1 | 2  => if swatch { "🟩" } else { "Low" },
    3 | 4 | 5  => if swatch { "🟨" } else { "Moderate" },
    6 | 7  => if swatch { "🟧" } else { "High" },
    8 | 9 | 10 => if swatch { "🟥" } else { "Very High" },
    _ => if swatch { "🟪" } else { "Extreme" },
  })
  
}

fn pressure_trend(trend: &PressureTrend) -> String {
  
  String::from(match trend {
    PressureTrend::Rising => "↑",
    PressureTrend::Falling => "↓",
    _ => "—"
  })
  
}

fn precip_type(precip: &PrecipitationType, daylight: bool) -> String {
  
  String::from(match precip {
    PrecipitationType::Hail => "🧊",
    PrecipitationType::Mixed => "🌂",
    PrecipitationType::Sleet => "⛆",
    PrecipitationType::Snow => "❄️",
    _ => if daylight { "😎" } else { "🌕" }
  })
  
}

fn condition_code(cond: &ConditionCode) -> String {
  
  String::from(match cond {
    ConditionCode::Blizzard => "Blizzard",
    ConditionCode::BlowingSnow => "Blowing Snow",
    ConditionCode::Breezy => "Breezy",
    ConditionCode::Clear => "Clear",
    ConditionCode::Cloudy => "Cloudy",
    ConditionCode::Drizzle => "Drizzle",
    ConditionCode::Dust => "Dust",
    ConditionCode::Flurries => "Flurries",
    ConditionCode::Fog => "Fog",
    ConditionCode::FreezingDrizzle => "Freezing Drizzle",
    ConditionCode::FreezingRain => "Freezing Rain",
    ConditionCode::Frigid => "Frigid",
    ConditionCode::Hail => "Hail",
    ConditionCode::Haze => "Haze",
    ConditionCode::HeavyRain => "HeavyRain",
    ConditionCode::HeavySnow => "HeavySnow",
    ConditionCode::Hot => "Hot",
    ConditionCode::Hurricane => "Hurricane",
    ConditionCode::IsolatedThunderstorms => "Isolated Thunderstorms",
    ConditionCode::MixedRainAndSleet => "Mixed Rain & Sleet",
    ConditionCode::MixedRainAndSnow => "Mixed Rain & Snow",
    ConditionCode::MixedRainfall => "MixedRainfall",
    ConditionCode::MixedSnowAndSleet => "Mixed Snow & Sleet",
    ConditionCode::MostlyClear => "Mostly Clear",
    ConditionCode::MostlyCloudy => "Mostly Cloudy",
    ConditionCode::PartlyCloudy => "Partly Cloudy",
    ConditionCode::Rain => "Rain",
    ConditionCode::ScatteredShowers => "Scattered Showers",
    ConditionCode::ScatteredSnowShowers => "Scattered Snow Showers",
    ConditionCode::ScatteredThunderstorms => "Scattered Thunderstorms",
    ConditionCode::SevereThunderstorm => "Severe Thunderstorm",
    ConditionCode::Showers => "Showers",
    ConditionCode::Sleet => "Sleet",
    ConditionCode::Smoke => "Smoke",
    ConditionCode::Snow => "Snow",
    ConditionCode::SnowShowers => "Snow Showers",
    ConditionCode::Thunderstorm => "Thunderstorm",
    ConditionCode::Tornado => "Tornado",
    ConditionCode::TropicalStorm => "Tropical Storm",
    ConditionCode::Windy => "Windy"
  })
  
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
  
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
  
  // println!("{}\n{}", claims, header);
  
  let token = encode(&header, &claims, &alg).expect("Error creating JWT");
  
  let latitude = 43.2;
  let longitude = -70.8;
  let language = "en";
  let tzone = lookup(latitude, longitude).unwrap();
  
  let utc_now = Utc::now();
  let utc_now_fmt = utc_now.to_rfc3339_opts(SecondsFormat::Secs, true);

  let url = format!(
    "https://weatherkit.apple.com/api/v1/weather/{}/{}/{}?timezone={}&dataSets=currentWeather,forecastDaily,forecastHourly,weatherAlerts&dailyStart={}&hourlyStart={}", 
    language, latitude, longitude, tzone, utc_now_fmt, utc_now_fmt
  );
  
  let client = reqwest::Client::new();
  
  let call = client
    .get(url)
    .header("Authorization", format!("Bearer {}", token))
    .header("Accept", "application/json");
  
  let resp = call.send().await?.json::<WeatherKitWeather>().await?;
  
  let num = NumberFormat::new();
  
  println!("{} daily forecast for ({}, {}) as of {}\n", APPLE_WEATHER_TRADEMARK, resp.current_weather.metadata.latitude, resp.current_weather.metadata.longitude, resp.current_weather.as_of);
  
  println!(" Conditions: {}",          format!("{:?}", resp.current_weather.condition_code));
  println!("Temperature: {}°F",        num.format(".1f", c_to_f(resp.current_weather.temperature)));
  println!(" Feels like: {}°F",        num.format(".1f", c_to_f(resp.current_weather.temperature_apparent)));
  println!("  Dew Point: {}°F",        num.format(".1f", c_to_f(resp.current_weather.temperature_dew_point)));
  println!("       Wind: {} mph",      num.format(".0f", kmph_to_mph(resp.current_weather.wind_speed)));
  println!("   Pressure: {} mb ({})",  num.format(".0f", resp.current_weather.pressure), format!("{:?}", resp.current_weather.pressure_trend));
  println!(" Visibility: {} miles",    meters_to_miles(resp.current_weather.wind_speed) as i64);
  println!("   UV Index: {} {}",       uv_label(resp.current_weather.uv_index, true), uv_label(resp.current_weather.uv_index, false));
  println!();

  let hrs = resp.forecast_hourly.hours;
  
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

  let tz: Tz = tzone.parse().unwrap();
   
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
      pressure_trend(&hour.pressure_trend),
      precip_type(&hour.precipitation_type, hour.daylight),
      condition_code(&hour.condition_code).pad_to_width(max_hr_condiiton_len),
      uv_label(hour.uv_index, true)
    );

    day_set.insert(printed_str.to_string());
    
  }
  
  Ok(())
  
}
