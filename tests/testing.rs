use chrono::{Local, Utc};
use triangular_earth_calendar::{self, DateTime, Day, Errors, Month, Time, Year};

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn past_date() {
    let dt = chrono::DateTime::parse_from_rfc3339("1990-02-13T09:11:33-07:00").expect("Fixed");
    let r = DateTime::from_datetime(dt);
    let n = DateTime::new(
        Year::new(-11),
        Month::new(1),
        Day::new(8),
        Time::new(38302),
        Some(*Local::now().fixed_offset().offset()),
    );
    assert_eq!(r, n)
}

#[test]
fn future_date() {
    let dt = chrono::DateTime::parse_from_rfc3339("2084-12-30T20:01:00-00:00").expect("Fixed");
    let r = DateTime::from_datetime(dt);
    let n = DateTime::new(
        Year::new(83),
        Month::new(10),
        Day::new(5),
        Time::new(83402),
        Some(*Utc::now().fixed_offset().offset()),
    );
    assert_eq!(r, n)
}
