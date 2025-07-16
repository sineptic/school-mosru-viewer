use std::{collections::BTreeMap, time::Instant};

use anyhow::Context;

use crate::{api::ApiClient, time::Date, types::schedule::LessonSchedule};

pub mod api;
#[allow(unused)]
mod raw_types;
pub mod time;
pub mod types;

mod database;

mod localfirst;

fn display_schedule(mut lessons: Vec<LessonSchedule>) {
    println!("=======================");
    lessons.sort_by(|a, b| a.date.cmp(&b.date).then(a.begin_time.cmp(&b.begin_time)));
    let mut date = None;
    for lesson in lessons {
        if Some(lesson.date) != date {
            println!();
            println!("{}", lesson.date);
            date = Some(lesson.date);
        }
        if let Some(room) = lesson.room_number {
            print!("{room:3}: ");
        } else {
            print!("___  ");
        }
        println!("{}", lesson.subject_name);
    }
    println!("=======================");
}

fn main() -> anyhow::Result<()> {
    let student_id = 31823383;
    dotenvy::dotenv().context("failed to load '.env' file")?;
    let state = localfirst::State::open(
        std::env::var("MOSRU_BEARER").context("failed to read 'MOSRU_BEARER' env variable")?,
        student_id,
    )
    .context("failed to load application state")?;
    let a = state.schedule(
        Date {
            year: 2024,
            month: 9,
            day: 2,
        },
        Date {
            year: 2024,
            month: 9,
            day: 3,
        },
    )?;
    display_schedule(a);
    while !state.should_update() {
        std::thread::sleep_ms(100);
    }
    let a = state.schedule(
        Date {
            year: 2024,
            month: 9,
            day: 2,
        },
        Date {
            year: 2024,
            month: 9,
            day: 3,
        },
    )?;
    display_schedule(a);
    std::thread::sleep_ms(5000);
    assert!(state.should_update());
    let b = state.schedule(
        Date {
            year: 2024,
            month: 9,
            day: 2,
        },
        Date {
            year: 2024,
            month: 9,
            day: 3,
        },
    )?;
    display_schedule(b);
    // std::fs::write("a.json", serde_json::to_string_pretty(&a).unwrap()).unwrap();
    // std::fs::write("b.json", serde_json::to_string_pretty(&b).unwrap()).unwrap();
    // dbg!(a, b);

    // select * from lesson_schedules order by date, begin_time limit 20;
    // use database::*;
    // let schedule: Vec<types::schedule::LessonSchedule> =
    //     serde_json::from_str(&std::fs::read_to_string("schedule.json")?)?;
    // let mut db = Database::open()?;
    // let now = Instant::now();
    // db.transaction(|tr| {
    //     tr.store_lesson_schedules(schedule)?;
    //     Ok(())
    // })?;
    // let elapsed = now.elapsed();
    // dbg!(elapsed);

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

    // let mut stmt = db.connection.prepare(
    //     "
    //     SELECT schedule_item_id, lesson_type
    //     FROM lesson_schedules
    //     ORDER BY date, begin_time
    //     LIMIT 10
    //     ",
    // )?;
    // let details = stmt
    //     .query_map((), |row| {
    //         Ok((row.get_unwrap::<_, u64>(0), row.get_unwrap(1)))
    //     })?
    //     .collect::<Result<Vec<_>, _>>()?
    //     .into_iter()
    //     .filter(|(id, _)| *id != 464137127)
    //     .map(|(id, ty)| {
    //         client.trigger_endpoint(api::LessonScheduleItems {
    //             student_id,
    //             schedule_item_id: id,
    //             lesson_type: ty,
    //         })
    //     })
    //     .collect::<Result<Vec<_>, _>>()?;
    // drop(stmt);
    // db.transaction(|tr| {
    //     for lesson_details in details {
    //         tr.add_room_for_lesson_schedule(
    //             lesson_details.id,
    //             lesson_details.room_number.parse().unwrap(),
    //         )?;
    //     }
    //     Ok(())
    // })?;

    // let endpoint = api::LessonScheduleItems {
    //     student_id,
    //     schedule_item_id: 464137127,
    //     lesson_type: LessonEducationType::Normal,
    // };
    // let response = client.trigger_endpoint(endpoint)?;
    // dbg!(response);
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
