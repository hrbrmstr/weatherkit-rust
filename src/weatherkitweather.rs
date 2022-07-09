// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_derive;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherKitWeather {
    #[serde(rename = "currentWeather")]
    pub current_weather: Option<CurrentWeather>,

    #[serde(rename = "forecastDaily")]
    pub forecast_daily: Option<ForecastDaily>,

    #[serde(rename = "forecastHourly")]
    pub forecast_hourly: Option<ForecastHourly>,

    #[serde(rename = "forecastNextHour")]
    pub forecast_next_hour: Option<ForecastNextHour>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "asOf")]
    pub as_of: Option<String>,

    #[serde(rename = "cloudCover")]
    pub cloud_cover: Option<f64>,

    #[serde(rename = "conditionCode")]
    pub condition_code: Option<ConditionCode>,

    #[serde(rename = "daylight")]
    pub daylight: Option<bool>,

    #[serde(rename = "humidity")]
    pub humidity: Option<f64>,

    #[serde(rename = "precipitationIntensity")]
    pub precipitation_intensity: Option<f64>,

    #[serde(rename = "pressure")]
    pub pressure: Option<f64>,

    #[serde(rename = "pressureTrend")]
    pub pressure_trend: Option<PressureTrend>,

    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,

    #[serde(rename = "temperatureApparent")]
    pub temperature_apparent: Option<f64>,

    #[serde(rename = "temperatureDewPoint")]
    pub temperature_dew_point: Option<f64>,

    #[serde(rename = "uvIndex")]
    pub uv_index: Option<i64>,

    #[serde(rename = "visibility")]
    pub visibility: Option<f64>,

    #[serde(rename = "windDirection")]
    pub wind_direction: Option<i64>,

    #[serde(rename = "windGust")]
    pub wind_gust: Option<f64>,

    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<f64>,

    #[serde(rename = "forecastStart")]
    pub forecast_start: Option<String>,

    #[serde(rename = "precipitationAmount")]
    pub precipitation_amount: Option<f64>,

    #[serde(rename = "precipitationChance")]
    pub precipitation_chance: Option<f64>,

    #[serde(rename = "precipitationType")]
    pub precipitation_type: Option<PrecipitationType>,

    #[serde(rename = "snowfallIntensity")]
    pub snowfall_intensity: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "attributionURL")]
    pub attribution_url: Option<String>,

    #[serde(rename = "expireTime")]
    pub expire_time: Option<String>,

    #[serde(rename = "latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "readTime")]
    pub read_time: Option<String>,

    #[serde(rename = "reportedTime")]
    pub reported_time: Option<String>,

    #[serde(rename = "units")]
    pub units: Option<String>,

    #[serde(rename = "version")]
    pub version: Option<i64>,

    #[serde(rename = "language")]
    pub language: Option<String>,

    #[serde(rename = "providerName")]
    pub provider_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastDaily {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "days")]
    pub days: Option<Vec<Day>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    #[serde(rename = "forecastStart")]
    pub forecast_start: Option<String>,

    #[serde(rename = "forecastEnd")]
    pub forecast_end: Option<String>,

    #[serde(rename = "conditionCode")]
    pub condition_code: Option<String>,

    #[serde(rename = "maxUvIndex")]
    pub max_uv_index: Option<i64>,

    #[serde(rename = "moonPhase")]
    pub moon_phase: Option<MoonPhase>,

    #[serde(rename = "moonrise")]
    pub moonrise: Option<String>,

    #[serde(rename = "moonset")]
    pub moonset: Option<String>,

    #[serde(rename = "precipitationAmount")]
    pub precipitation_amount: Option<f64>,

    #[serde(rename = "precipitationChance")]
    pub precipitation_chance: Option<f64>,

    #[serde(rename = "precipitationType")]
    pub precipitation_type: Option<PrecipitationType>,

    #[serde(rename = "snowfallAmount")]
    pub snowfall_amount: Option<i64>,

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
    pub temperature_max: Option<f64>,

    #[serde(rename = "temperatureMin")]
    pub temperature_min: Option<f64>,

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
    pub forecast_start: Option<String>,

    #[serde(rename = "forecastEnd")]
    pub forecast_end: Option<String>,

    #[serde(rename = "cloudCover")]
    pub cloud_cover: Option<f64>,

    #[serde(rename = "conditionCode")]
    pub condition_code: Option<ConditionCode>,

    #[serde(rename = "humidity")]
    pub humidity: Option<f64>,

    #[serde(rename = "precipitationAmount")]
    pub precipitation_amount: Option<f64>,

    #[serde(rename = "precipitationChance")]
    pub precipitation_chance: Option<f64>,

    #[serde(rename = "precipitationType")]
    pub precipitation_type: Option<PrecipitationType>,

    #[serde(rename = "snowfallAmount")]
    pub snowfall_amount: Option<i64>,

    #[serde(rename = "windDirection")]
    pub wind_direction: Option<i64>,

    #[serde(rename = "windSpeed")]
    pub wind_speed: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastHourly {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "hours")]
    pub hours: Option<Vec<CurrentWeather>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastNextHour {
    #[serde(rename = "name")]
    pub name: Option<String>,

    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,

    #[serde(rename = "summary")]
    pub summary: Option<Vec<Minute>>,

    #[serde(rename = "forecastStart")]
    pub forecast_start: Option<String>,

    #[serde(rename = "forecastEnd")]
    pub forecast_end: Option<String>,

    #[serde(rename = "minutes")]
    pub minutes: Option<Vec<Minute>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minute {
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,

    #[serde(rename = "precipitationChance")]
    pub precipitation_chance: Option<i64>,

    #[serde(rename = "precipitationIntensity")]
    pub precipitation_intensity: Option<i64>,

    #[serde(rename = "condition")]
    pub condition: Option<PrecipitationType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConditionCode {
    #[serde(rename = "Clear")]
    Clear,

    #[serde(rename = "Cloudy")]
    Cloudy,

    #[serde(rename = "MostlyClear")]
    MostlyClear,

    #[serde(rename = "MostlyCloudy")]
    MostlyCloudy,

    #[serde(rename = "PartlyCloudy")]
    PartlyCloudy,

    #[serde(rename = "Rain")]
    Rain,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrecipitationType {
    #[serde(rename = "clear")]
    Clear,

    #[serde(rename = "rain")]
    Rain,
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
    #[serde(rename = "full")]
    Full,

    #[serde(rename = "waningGibbous")]
    WaningGibbous,

    #[serde(rename = "waxingGibbous")]
    WaxingGibbous,
}
