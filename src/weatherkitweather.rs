use serde::{Serialize, Deserialize};

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

 #[serde(rename = "weatherAlerts")]
  pub weather_alerts: Option<WeatherAlerts>,
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
  pub daylight: Option<bool>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "humidity")]
  pub humidity: f64,
  
  #[serde(rename = "precipitationIntensity")]
  pub precipitation_intensity: f64,
  
  #[serde(rename = "pressure")]
  pub pressure: f64,
  
  #[serde(rename = "pressureTrend")]
  pub pressure_trend: Option<PressureTrend>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "temperature")]
  pub temperature: f64,
  
  #[serde(rename = "temperatureApparent")]
  pub temperature_apparent: f64,
  
  #[serde(rename = "temperatureDewPoint")]
  pub temperature_dew_point: Option<f64>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "uvIndex")]
  pub uv_index: f64,
  
  #[serde(rename = "visibility")]
  pub visibility: f64,
  
  #[serde(rename = "windDirection")]
  pub wind_direction: Option<f64>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "windGust")]
  pub wind_gust: Option<f64>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "windSpeed")]
  pub wind_speed: f64,
  
  #[serde(rename = "forecastStart")]
  pub forecast_start: String,
  
  #[serde(rename = "precipitationAmount")]
  pub precipitation_amount: Option<f64>, // TODO: CAN BE OPTIONAL
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationType")]
  pub precipitation_type: PrecipitationType,
  
  #[serde(rename = "snowfallIntensity")]
  pub snowfall_intensity: Option<f64>, // TODO: CAN BE OPTIONAL
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
  pub units: Option<String>,
  
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
  pub condition_code: ConditionCode,
  
  #[serde(rename = "maxUvIndex")]
  pub max_uv_index: f64,
  
  #[serde(rename = "moonPhase")]
  pub moon_phase: Option<MoonPhase>,
  
  #[serde(rename = "moonrise")]
  pub moonrise: Option<String>,
  
  #[serde(rename = "moonset")]
  pub moonset: Option<String>,
  
  #[serde(rename = "precipitationAmount")]
  pub precipitation_amount: f64,
  
  #[serde(rename = "precipitationChance")]
  pub precipitation_chance: f64,
  
  #[serde(rename = "precipitationType")]
  pub precipitation_type: PrecipitationType,
  
  #[serde(rename = "snowfallAmount")]
  pub snowfall_amount: f64,
  
  #[serde(rename = "solarMidnight")]
  pub solar_midnight: Option<String>,
  
  #[serde(rename = "solarNoon")]
  pub solar_noon: Option<String>,
  
  #[serde(rename = "sunrise")]
  pub sunrise: Option<String>,
  
  #[serde(rename = "sunriseCivil")]
  pub sunrise_civil: Option<String>,
  
  #[serde(rename = "sunriseNautical")]
  pub sunrise_nautical: Option<String>,
  
  #[serde(rename = "sunriseAstronomical")]
  pub sunrise_astronomical: Option<String>,
  
  #[serde(rename = "sunset")]
  pub sunset: Option<String>,
  
  #[serde(rename = "sunsetCivil")]
  pub sunset_civil: Option<String>,
  
  #[serde(rename = "sunsetNautical")]
  pub sunset_nautical: Option<String>,
  
  #[serde(rename = "sunsetAstronomical")]
  pub sunset_astronomical: Option<String>,
  
  #[serde(rename = "temperatureMax")]
  pub temperature_max: f64,
  
  #[serde(rename = "temperatureMin")]
  pub temperature_min: f64,
  
  #[serde(rename = "daytimeForecast")]
  pub daytime_forecast: Option<Forecast>,
  
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
  pub wind_direction: Option<f64>,
  
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

  #[serde(rename = "Foggy")]
  Foggy,
  
  #[serde(rename = "Haze")]
  Haze,
  
  #[serde(rename = "MostlyClear")]
  MostlyClear,
  
  #[serde(rename = "MostlyCloudy")]
  MostlyCloudy,
  
  #[serde(rename = "PartlyCloudy")]
  PartlyCloudy,
  
  #[serde(rename = "Thunderstorms")]
  Thunderstorms,
  
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
  
  #[serde(rename="secondQuarter")]
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


pub fn uv_label(uv_index: f64, swatch: bool) -> String {
  
  String::from(match uv_index as i64 {
    0 | 1 | 2  => if swatch { "🟩" } else { "Low" },
    3 | 4 | 5  => if swatch { "🟨" } else { "Moderate" },
    6 | 7  => if swatch { "🟧" } else { "High" },
    8 | 9 | 10 => if swatch { "🟥" } else { "Very High" },
    _ => if swatch { "🟪" } else { "Extreme" },
  })
  
}

pub fn pressure_trend(trend: &PressureTrend) -> String {
  
  String::from(match trend {
    PressureTrend::Rising => "↑",
    PressureTrend::Falling => "↓",
    _ => "—"
  })
  
}

// convert precip type to an emoji
pub fn precip_type(precip: &PrecipitationType, daylight: bool) -> String {
  
  String::from(match precip {
    PrecipitationType::Hail => "🧊",
    PrecipitationType::Mixed => "🌂",
    PrecipitationType::Sleet => "⛆",
    PrecipitationType::Snow => "❄️ ",
    _ => if daylight { "😎" } else { "🌕" }
  })
  
}


pub fn condition_code(cond: &ConditionCode) -> String {
  
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
    ConditionCode::Foggy => "Foggy",
    ConditionCode::FreezingDrizzle => "Freezing Drizzle",
    ConditionCode::FreezingRain => "Freezing Rain",
    ConditionCode::Frigid => "Frigid",
    ConditionCode::Hail => "Hail",
    ConditionCode::Haze => "Haze",
    ConditionCode::HeavyRain => "Heavy Rain",
    ConditionCode::HeavySnow => "Heavy Snow",
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
    ConditionCode::Thunderstorms => "Thunderstorms",
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherAlerts {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "metadata")]
    pub metadata: Metadata,

    #[serde(rename = "detailsUrl")]
    pub details_url: String,

    #[serde(rename = "alerts")]
    pub alerts: Vec<Alert>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "areaId")]
    pub area_id: String,

    #[serde(rename = "areaName")]
    pub area_name: String,

    #[serde(rename = "countryCode")]
    pub country_code: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "effectiveTime")]
    pub effective_time: String,

    #[serde(rename = "expireTime")]
    pub expire_time: String,

    #[serde(rename = "issuedTime")]
    pub issued_time: String,

    #[serde(rename = "eventEndTime")]
    pub event_end_time: String,

    #[serde(rename = "detailsUrl")]
    pub details_url: String,

    #[serde(rename = "precedence")]
    pub precedence: i64,

    #[serde(rename = "severity")]
    pub severity: Severity,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "eventSource")]
    pub event_source: String,

    #[serde(rename = "urgency")]
    pub urgency: String,

    #[serde(rename = "certainty")]
    pub certainty: String,

    #[serde(rename = "importance")]
    pub importance: String,

    #[serde(rename = "responses")]
    pub responses: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
  #[serde(rename="extreme")]
  Extreme,

  #[serde(rename="severe")]
  Severe,
  
  #[serde(rename="moderate")]
  Moderate,
  
  #[serde(rename="minor")]
  Minor,
  
  #[serde(rename="unknown")]
  Unknown,
  
}


pub fn severity_trans(severity: Severity) -> String {

  String::from(match severity {
     Severity::Extreme => "🔴 Extreme (Extraordinary threat)",
      Severity::Severe => "🟠 Severe (Significant threat)",
    Severity::Moderate => "🟡 Moderate (Possible threat)",
       Severity::Minor => "🔵 Minor (Minimal or no known threat)",
     Severity::Unknown => r#"¯\_ (ツ)_/¯"#
  })

}
