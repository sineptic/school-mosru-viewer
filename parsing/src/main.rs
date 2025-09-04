use std::collections::BTreeMap;

use anyhow::Context;
use crossterm::ExecutableCommand;

use crate::{time::Date, types::schedule::LessonSchedule};

pub mod api;
#[allow(unused)]
mod raw_types;
pub mod time;
pub mod types;

mod database;

mod localfirst;

fn main() -> anyhow::Result<()> {
    // todo: initialize logger

    let student_id = 31823383;
    dotenvy::dotenv().context("failed to load '.env' file")?;
    let state = localfirst::State::open(
        std::env::var("MOSRU_BEARER").context("failed to read 'MOSRU_BEARER' env variable")?,
        student_id,
    )
    .context("failed to load application state")?;
    let from = Date {
        year: 2025,
        month: 9,
        day: 1,
    };
    let to = Date {
        year: 2026,
        month: 9,
        day: 3,
    };
    loop {
        let schedule = state.schedule(from, to)?;
        display_schedule(schedule);
        while !state.should_update() {
            std::thread::sleep_ms(100);
        }
    }
}

fn display_schedule(mut lessons: Vec<LessonSchedule>) {
    std::io::stdout()
        .execute(crossterm::terminal::Clear(
            crossterm::terminal::ClearType::All,
        ))
        .unwrap()
        .execute(crossterm::cursor::MoveTo(0, 0))
        .unwrap();
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

fn _register(storage: &mut BTreeMap<String, u32>, key: impl ToString) {
    *storage.entry(key.to_string()).or_default() += 1;
}

fn _register_maybe(storage: &mut BTreeMap<String, u32>, key: Option<impl ToString>) {
    _register(storage, key.map(|x| x.to_string()).unwrap_or("none".into()));
}

fn _all_possible_dates() -> Vec<Date> {
    let mut dates = Vec::new();
    let start = Date {
        year: 2025,
        month: 9,
        day: 1,
    };
    let end = Date {
        year: 2026,
        month: 5,
        day: 31,
    };
    Date::iterate_days_inclusive(start, end, |date| dates.push(date));
    dates
}
