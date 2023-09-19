use chrono::{Datelike, FixedOffset, Local, Offset, Timelike, Utc};
use std::{fmt, time};
use std::{
    num::ParseIntError,
    ops::{Add, Sub},
    str::FromStr,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    //#[error("Set Logger Error")]
    //SetLogger(#[from] log::SetLoggerError),
    #[error("Parse chrono datetime error")]
    ParseDateTime(#[from] chrono::ParseError),
    #[error("Error when attempting to parse string to int")]
    ParseInt(#[from] ParseIntError),
    #[error("Error when perform std IO")]
    Io(#[from] std::io::Error),
    #[error("Generic Error:\n{0}")]
    Generic(String),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Year(i32);
impl Year {
    pub fn new(year: i32) -> Self {
        Year(year)
    }
    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let greg_year = dt.year();
        Year(greg_year - 2001)
    }
    pub fn is_leap_year(&self) -> bool {
        let y = self.0 + 2001;
        (y % 4) == 0 && (y % 100) != 0 || (y % 400) == 0
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Year {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Year(self.0 + other.0)
    }
}

impl Sub for Year {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Year(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Month(u32);
impl Month {
    pub fn new(month: u32) -> Self {
        Month(month)
    }
    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let year_days = dt
            .format("%j")
            .to_string()
            .parse::<u32>()
            .expect("Failed to parse year day");
        Month(year_days / 36)
    }
}
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 == 10 {
            write!(f, "A")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Add for Month {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Month(self.0 + other.0)
    }
}

impl Sub for Month {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Month(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Day(u32);
impl Day {
    pub fn new(day: u32) -> Self {
        Day(day)
    }
    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let year_days = dt
            .format("%j")
            .to_string()
            .parse::<u32>()
            .expect("Failed to parse year day");
        if year_days == 1 {
            Day(0)
        } else {
            let res = (year_days / 36) * 36;
            Day(year_days - res)
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for Day {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Day(self.0 + other.0)
    }
}

impl Sub for Day {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Day(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time(u32);
impl Time {
    pub fn new(time: u32) -> Self {
        Time(time)
    }
    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let hour = dt.hour();
        let minute = dt.minute();
        let second = dt.second();
        Time::from_hms(hour, minute, second)
    }

    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Self {
        let res = ((hour * 3600) + (minute * 60) + second) as f32 * 1.15741;
        // let res = ((hour as f64 / 24.0)
        //     + (minute as f64 / 1440.0)
        //     + (second as f64 / 86400.0) * 100000.0);
        Time(res as u32)
    }

    pub fn from_hour(hour: u32) -> Self {
        Time(((hour * 3600) as f32 * 1.15741) as u32)
    }

    pub fn now() -> Self {
        let now: chrono::DateTime<FixedOffset> = Local::now().fixed_offset();
        Time::from_datetime(now)
    }
}
impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strng = format!("{:0>5}", self.0);
        let remove_trailing = strng.strip_suffix('0');
        if let Some(rt) = remove_trailing {
            write!(f, "{}", rt)
        } else {
            write!(f, "{}", strng)
        }
    }
}

impl FromStr for Time {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strng = s.strip_prefix(':').unwrap_or(s);
        Ok(Time::new(strng.parse::<u32>()?))
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Time(self.0 + other.0)
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Time(self.0 - other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub fn new(year: Year, month: Month, day: Day) -> Self {
        Date { year, month, day }
    }
    pub fn tec_epoch() -> Self {
        Date::new(Year::new(0), Month::new(0), Day::new(0))
    }
    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let year = Year::from_datetime(dt);
        let month = Month::from_datetime(dt);
        let day = Day::from_datetime(dt);
        Date::new(year, month, day)
    }
}

impl FromStr for Date {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l: Vec<&str> = s.split('.').collect();
        let year = Year::new(l[0].parse::<i32>()?);
        let month_num = match l[1].parse::<u32>() {
            Ok(i) => i,
            Err(_) => {
                if l[1] == "A" {
                    10
                } else {
                    return Err(Errors::Generic(format!("{} month is not valid", l[1])));
                }
            }
        };
        let month = Month::new(month_num);
        let day = Day::new(l[2].parse::<u32>()?);
        Ok(Date::new(year, month, day))
    }
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.year, self.month, self.day)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime {
    date: Date,
    time: Time,
    offset: FixedOffset,
}

impl DateTime {
    pub fn new(date: Date, time: Time, offset_opt: Option<FixedOffset>) -> Self {
        let offset = if let Some(off) = offset_opt {
            off
        } else {
            *Local::now().fixed_offset().offset()
        };
        DateTime { date, time, offset }
    }

    pub fn from_datetime(dt: chrono::DateTime<FixedOffset>) -> Self {
        let date = Date::from_datetime(dt);
        let time = Time::from_datetime(dt);
        let offset = dt.offset();
        DateTime::new(date, time, Some(*offset))
    }

    pub fn now() -> Self {
        let now: chrono::DateTime<FixedOffset> = Local::now().fixed_offset();
        DateTime::from_datetime(now)
    }

    pub fn tec_epoch() -> Self {
        let date = Date::new(Year(0), Month(0), Day(0));
        DateTime::new(date, Time(0), Some(Utc.fix()))
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local_offset = *Local::now().fixed_offset().offset();
        if local_offset == self.offset {
            write!(f, "{}:{}", self.date, self.time)
        } else {
            write!(f, "{}:{}@{}", self.date, self.time, self.offset)
        }
    }
}

fn str2offset(s: &str) -> Result<FixedOffset, Errors> {
    if s.contains(':') {
        let strng = s.replace(':', "");
        let num = strng.parse::<i32>()?;
        let secs = num * 60;
        let fo = FixedOffset::west_opt(secs).unwrap();
        Ok(fo)
    } else {
        let num = s.parse::<i32>()?;
        let secs = num * 60;
        let fo = FixedOffset::west_opt(secs).unwrap();
        Ok(fo)
    }
}

impl FromStr for DateTime {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offset = if s.contains('@') {
            let strng: Vec<&str> = s.split('@').collect();
            str2offset(strng[1])?
        } else {
            *Local::now().fixed_offset().offset()
        };

        if s.contains(':') && !s.contains('-') {
            let splt: Vec<&str> = s.split(':').collect();
            let date = Date::from_str(splt[0])?;
            let time = Time::from_str(splt[1])?;
            Ok(DateTime::new(date, time, Some(offset)))
        } else if s.contains('.') {
            let date = Date::from_str(s)?;
            let time = Time::new(50000);
            Ok(DateTime::new(date, time, Some(offset)))
        } else {
            match dateparser::parse(s) {
                Ok(dt) => Ok(DateTime::from_datetime(dt.fixed_offset())),
                Err(_) => Err(Errors::Generic(format!("{s} is not a gregorian datetime"))),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub struct Duration(u64);

impl Duration {
    pub fn new(fracs: u64) -> Self {
        Duration(fracs)
    }
    pub fn from_secs(secs: u64) -> Self {
        Duration((secs as f64 * 1.1574) as u64)
    }
    pub fn to_secs(&self) -> u64 {
        (self.0 as f64 / 1.1574) as u64
    }
    pub fn from_std_dur(dur: time::Duration) -> Self {
        Duration::from_secs(dur.as_secs())
    }

    pub fn to_std_dur(&self) -> time::Duration {
        time::Duration::from_secs(self.to_secs())
    }
}

impl FromStr for Duration {
    type Err = Errors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();
        if lower.ends_with('s') {
            let strp = lower.strip_suffix('s').unwrap(); // Can't fail
            let secs = strp.parse::<u64>()?;
            Ok(Duration::from_secs(secs))
        } else if lower.ends_with('f') {
            let strp = lower.strip_suffix('f').unwrap();
            let fracs = strp.parse::<u64>()?;
            Ok(Duration::new(fracs))
        } else {
            let fracs = s.parse::<u64>()?;
            Ok(Duration::new(fracs))
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
