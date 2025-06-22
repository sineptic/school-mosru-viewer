use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub struct Date {
    year: u64,
    month: u64,
    day: u64,
}
impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let err = || serde::de::Error::custom("Date format should be 'year-mm-dd'.");
        let input = String::deserialize(deserializer)?;
        if input.len() != 10 {
            return Err(err());
        }
        let [y1, y2, y3, y4, dash1, m1, m2, dash2, d1, d2] = input.as_bytes().try_into().unwrap();
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
            return Err(err());
        }
        let year = (y1 - b'0') as u64 * 1000
            + (y2 - b'0') as u64 * 100
            + (y3 - b'0') as u64 * 10
            + (y4 - b'0') as u64;
        let month = (m1 - b'0') as u64 * 10 + (m2 - b'0') as u64;
        let day = (d1 - b'0') as u64 * 10 + (d2 - b'0') as u64;
        if !((1..=12).contains(&month) && (1..=31).contains(&day)) {
            return Err(err());
        }

        Ok(Self { year, month, day })
    }
}
impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Time {
    hours: u64,
    minutes: u64,
}
impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let err = || serde::de::Error::custom("Time format should be 'hh:mm'.");
        let input = String::deserialize(deserializer)?;
        if input.len() != 5 {
            return Err(err());
        }
        let [h1, h2, column, m1, m2] = input.as_bytes().try_into().unwrap();
        if !(h1.is_ascii_digit()
            && h2.is_ascii_digit()
            && column == b':'
            && m1.is_ascii_digit()
            && m2.is_ascii_digit())
        {
            return Err(err());
        }
        let hours = (h1 - b'0') as u64 * 10 + (h2 - b'0') as u64;
        let minutes = (m1 - b'0') as u64 * 10 + (m2 - b'0') as u64;
        if !((0..24).contains(&hours) && (0..60).contains(&minutes)) {
            return Err(err());
        }

        Ok(Self { hours, minutes })
    }
}
impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:02}:{:02}", self.hours, self.minutes))
    }
}
