use chrono::{DateTime, TimeZone, Utc};

#[cfg(test)]
pub fn string_to_utc(date_string: &str) -> DateTime<Utc> {
    let datetime = DateTime::parse_from_str(date_string, "%Y-%m-%d %H:%M:%S %z %Z").unwrap();
    Utc.from_local_datetime(&datetime.naive_utc()).unwrap()
}

#[cfg(test)]
pub fn round_float(num: f64, precision: i32) -> f64 {
    let multiplier = 10.0_f64.powi(precision);
    (num * multiplier).round() / multiplier
}
