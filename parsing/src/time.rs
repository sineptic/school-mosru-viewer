use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: u64,
    pub month: u64,
    pub day: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hours: u64,
    pub minutes: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl Date {
    pub fn is_year_leap(year: u64) -> bool {
        year.is_multiple_of(400) || year.is_multiple_of(4) && !year.is_multiple_of(100)
    }
    pub fn days_in_month(month: u64, is_leap: bool) -> u64 {
        match month {
            1 => 31,
            2 => 28 + is_leap as u64,
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => unreachable!(),
        }
    }
    pub fn is_correct(&self) -> bool {
        self.month >= 1
            && self.month <= 12
            && self.day >= 1
            && self.day <= Self::days_in_month(self.month, Self::is_year_leap(self.year))
    }
    pub fn next_day(mut self) -> Self {
        assert!(self.is_correct());
        self.day += 1;
        if !self.is_correct() {
            self.month += 1;
            self.day = 1;
            if !self.is_correct() {
                self.year += 1;
                self.month = 1;
            }
        }
        self
    }
    pub fn prev_day(mut self) -> Self {
        assert!(self.is_correct());
        self.day -= 1;
        if !self.is_correct() {
            self.month -= 1;
            if !self.is_correct() {
                self.year -= 1;
                self.month = 12;
            }
            self.day = Self::days_in_month(self.month, Self::is_year_leap(self.year));
        }
        self
    }
    pub fn iterate_days_inclusive(start: Date, end: Date, mut callback: impl FnMut(Date)) {
        assert!(start.is_correct());
        assert!(end.is_correct());
        assert!(start <= end);
        let mut current = start;
        while current <= end {
            callback(current);
            current = current.next_day();
        }
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(self.is_correct());
        assert!(other.is_correct());
        self.year.cmp(&other.year).then_with(|| {
            self.month
                .cmp(&other.month)
                .then_with(|| self.day.cmp(&other.day))
        })
    }
}
impl FromStr for Date {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "Date format should be 'year-mm-dd'.";
        if s.len() != 10 {
            return Err(ERR);
        }
        let [y1, y2, y3, y4, dash1, m1, m2, dash2, d1, d2] = s.as_bytes().try_into().unwrap();
        if !(y1.is_ascii_digit()
            && y2.is_ascii_digit()
            && y3.is_ascii_digit()
            && y4.is_ascii_digit()
            && dash1 == b'-'
            && m1.is_ascii_digit()
            && m2.is_ascii_digit()
            && dash2 == b'-'
            && d1.is_ascii_digit()
            && d2.is_ascii_digit())
        {
            return Err(ERR);
        }
        let year = (y1 - b'0') as u64 * 1000
            + (y2 - b'0') as u64 * 100
            + (y3 - b'0') as u64 * 10
            + (y4 - b'0') as u64;
        let month = (m1 - b'0') as u64 * 10 + (m2 - b'0') as u64;
        let day = (d1 - b'0') as u64 * 10 + (d2 - b'0') as u64;
        if !((1..=12).contains(&month) && (1..=31).contains(&day)) {
            return Err(ERR);
        }

        Ok(Self { year, month, day })
    }
}

impl FromStr for Time {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERR: &str = "Time format should be 'hh:mm'.";
        if s.len() != 5 {
            return Err(ERR);
        }
        let [h1, h2, column, m1, m2] = s.as_bytes().try_into().unwrap();
        if !(h1.is_ascii_digit()
            && h2.is_ascii_digit()
            && column == b':'
            && m1.is_ascii_digit()
            && m2.is_ascii_digit())
        {
            return Err(ERR);
        }
        let hours = (h1 - b'0') as u64 * 10 + (h2 - b'0') as u64;
        let minutes = (m1 - b'0') as u64 * 10 + (m2 - b'0') as u64;
        if !((0..24).contains(&hours) && (0..60).contains(&minutes)) {
            return Err(ERR);
        }

        Ok(Self { hours, minutes })
    }
}
impl FromStr for DateTime {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(" ")
            .or_else(|| s.split_once("T"))
            .ok_or("Date and time should be delimeted by ` ` or `T`")?;

        let date = Date::from_str(left)?;
        let time = right.parse().or_else(|_| {
            right
                .strip_suffix(":00")
                .ok_or("Time in DateTime should end with `:00`")
                .and_then(|x| x.parse())
        })?;
        Ok(Self { date, time })
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        input.parse().map_err(serde::de::Error::custom)
    }
}
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        input.parse().map_err(serde::de::Error::custom)
    }
}
impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = String::deserialize(deserializer)?;
        input.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
