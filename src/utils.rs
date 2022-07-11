/// Rescale a given integer value from one range to another
pub fn rescale_val(x: i64, from_min: i64, from_max: i64, to_min: i64, to_max: i64) -> i64 {
  (x - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

/// Convert Celsius to Fahrenheit
pub fn c_to_f(temp: f64) -> f64 {
  (9.0/5.0) * temp + 32.0
}

/// Convert meters to miles
pub fn meters_to_miles(meters: f64) -> i64 {
  (meters * 0.000621) as i64
}

/// Convert kilometers per hour to miles per hour
pub fn kmph_to_mph(kmph: f64) -> f64 {
  kmph * 0.539593 * 1.1507794
}
