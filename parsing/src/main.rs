use std::{collections::BTreeMap, time::Instant};

use crate::{api::ApiClient, time::Date};

mod api;
#[allow(unused)]
mod raw_types;
mod time;
mod types;

mod database;

fn main() -> anyhow::Result<()> {
    use database::*;
    let mut db = Database::open()?;
    let schedule: Vec<types::schedule::LessonSchedule> =
        serde_json::from_str(&std::fs::read_to_string("schedule.json")?)?;
    dbg!(schedule.len());
    let now = Instant::now();
    // let mut stmt = db.connection.prepare("select * from lesson_schedules")?;
    // let count = stmt.query(())?.count()?;
    db.transaction(|tr| {
        tr.store_lesson_schedules(schedule)?;
        Ok(())
    })?;
    let elapsed = now.elapsed();
    dbg!(elapsed);

    // dotenvy::dotenv()?;
    // let client = ApiClient::new(std::env::var("MOSRU_BEARER")?);
    // let student_id = 31823383;

    // let endpoint = api::Homework {
    //     student_id,
    //     from: Date {
    //         year: 2024,
    //         month: 9,
    //         day: 1,
    //     },
    //     to: Date {
    //         year: 2025,
    //         month: 5,
    //         day: 31,
    //     },
    // };
    // let response = client.trigger_endpoint(endpoint)?;
    // // dbg!(response);
    // // std::fs::write("homeworks.json", serde_json::to_string_pretty(&response)?)?;

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
