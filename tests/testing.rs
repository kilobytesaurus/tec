use chrono::{Local, Utc};
use triangular_earth_calendar::{self, Date, DateTime, Day, Errors, Month, Time, Year};

#[cfg(test)]
use pretty_assertions::assert_eq;

// Time Tests
#[test]
fn midnight() {
    let dt = chrono::DateTime::parse_from_rfc3339("2002-01-01T00:00:00-00:00").expect("Fixed");
    let from_dt = Time::from_datetime(dt);
    let from_hms = Time::from_hms(0, 0, 0);
    let from_hour = Time::from_hour(0);
    let correct = Time::new(0);
    assert_eq!(from_dt, correct);
    assert_eq!(from_hms, correct);
    assert_eq!(from_hour, correct);
}

#[test]
fn noon() {
    let dt = chrono::DateTime::parse_from_rfc3339("2002-01-01T12:00:00-00:00").expect("Fixed");
    let from_dt = Time::from_datetime(dt);
    let from_hms = Time::from_hms(12, 0, 0);
    let from_hour = Time::from_hour(12);
    let correct = Time::new(50000);
    assert_eq!(from_dt, correct, "From_DateTime method failed");
    assert_eq!(from_hms, correct, "From_Hms method failed");
    assert_eq!(from_hour, correct, "From_Hour method failed");
}

// Date Tests
#[test]
fn epoch() {
    let dt = chrono::DateTime::parse_from_rfc3339("2001-01-01T00:00:00-00:00").expect("Fixed");
    let from_dt = Date::from_datetime(dt);
    let correct = Date::new(Year::new(0), Month::new(0), Day::new(0));
    assert_eq!(from_dt, correct, "From_DateTime method failed");
}

// DateTime Tests
#[test]
fn past_date() {
    let dt = chrono::DateTime::parse_from_rfc3339("1990-02-13T09:11:33-07:00").expect("Fixed");
    let r = DateTime::from_datetime(dt);
    let past_date = Date::new(Year::new(-11), Month::new(1), Day::new(8));
    let past_time = Time::new(38302);
    let n = DateTime::new(
        past_date,
        past_time,
        Some(*Local::now().fixed_offset().offset()),
    );
    assert_eq!(r, n)
}

#[test]
fn future_date() {
    let dt = chrono::DateTime::parse_from_rfc3339("2084-12-30T20:01:00-00:00").expect("Fixed");
    let r = DateTime::from_datetime(dt);
    let future_time = Time::new(83402);
    let future_date = Date::new(Year::new(83), Month::new(10), Day::new(5));
    let n = DateTime::new(
        future_date,
        future_time,
        Some(*Utc::now().fixed_offset().offset()),
    );
    assert_eq!(r, n)
}
