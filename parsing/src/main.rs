use std::{collections::BTreeMap, time::Instant};

use crate::{api::ApiClient, raw_types::enums::LessonEducationType, time::Date};

pub mod api;
#[allow(unused)]
mod raw_types;
pub mod time;
pub mod types;

mod database;

fn main() -> anyhow::Result<()> {
    use database::*;
    let schedule: Vec<types::schedule::LessonSchedule> =
        serde_json::from_str(&std::fs::read_to_string("schedule.json")?)?;
    let mut db = Database::open()?;
    let now = Instant::now();
    db.transaction(|tr| {
        tr.store_lesson_schedules(schedule)?;
        Ok(())
    })?;
    let elapsed = now.elapsed();
    dbg!(elapsed);

    // let detailed_schedule: Vec<raw_types::details::LessonDetails> =
    //     serde_json::from_str(&std::fs::read_to_string("detailed_schedule.json")?)?;

    // db.transaction(|tr| {
    //     for lesson_schedule in detailed_schedule {
    //         tr.add_room_for_lesson_schedule(
    //             lesson_schedule.id,
    //             lesson_schedule.room_number.parse().unwrap(),
    //         )?;
    //     }
    //     Ok(())
    // })?;

    // let schedule: Vec<raw_types::details::LessonDetails> =
    //     serde_json::from_str(&std::fs::read_to_string("detailed_schedule.json")?)?;
    // for lesson_schedule in schedule {
    //     assert!(lesson_schedule.room_number.parse::<u32>().is_ok());
    // }

    // dotenvy::dotenv()?;
    // let client = ApiClient::new(std::env::var("MOSRU_BEARER")?);
    // let student_id = 31823383;

    // let endpoint = api::Schedule {
    //     student_id,
    //     dates: _all_possible_dates(),
    // };
    // let response = client.trigger_endpoint(endpoint)?;
    // // dbg!(response);
    // std::fs::write("schedule.json", serde_json::to_string_pretty(&response)?)?;

    Ok(())
}

fn _register(storage: &mut BTreeMap<String, u32>, key: impl ToString) {
    *storage.entry(key.to_string()).or_default() += 1;
}

fn _register_maybe(storage: &mut BTreeMap<String, u32>, key: Option<impl ToString>) {
    _register(storage, key.map(|x| x.to_string()).unwrap_or("none".into()));
}

fn _all_possible_dates() -> Vec<Date> {
    let mut dates = Vec::new();
    let start = Date {
        year: 2024,
        month: 9,
        day: 1,
    };
    let end = Date {
        year: 2025,
        month: 5,
        day: 31,
    };
    Date::iterate_days_inclusive(start, end, |date| dates.push(date));
    dates
}
