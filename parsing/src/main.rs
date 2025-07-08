use std::collections::BTreeMap;

use crate::{api::ApiClient, time::Date};

mod api;
#[allow(unused)]
mod raw_types;
mod time;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = ApiClient::new(std::env::var("MOSRU_BEARER").unwrap());
    let student_id = 31823383;

    let dates = all_possible_dates();

    let endpoint = api::Schedule { student_id, dates };
    let response = client.trigger_endpoint(endpoint).await?;

    let storage = &mut BTreeMap::<String, u32>::new();

    // for day in response.payload {
    //     for lesson in day.lessons {
    //         register_maybe(storage, lesson.bell_id);
    //     }
    // }

    dbg!(storage);

    Ok(())
}

fn register(storage: &mut BTreeMap<String, u32>, key: impl ToString) {
    *storage.entry(key.to_string()).or_default() += 1;
}

fn register_maybe(storage: &mut BTreeMap<String, u32>, key: Option<impl ToString>) {
    register(storage, key.map(|x| x.to_string()).unwrap_or("none".into()))
}

fn all_possible_dates() -> Vec<Date> {
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
