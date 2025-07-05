use std::time::Duration;

use api::ApiClient;

use crate::time::Date;

mod api;
mod raw_types;
mod time;
mod types;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = ApiClient::new(std::env::var("MOSRU_BEARER").unwrap());
    let student_id = 31823383;

    let dates = all_possible_dates();

    let endpoint = api::Schedule { student_id, dates };
    let response = client.trigger_endpoint(endpoint).await?;

    let schedule_ids = response.into_iter().map(|x| x.schedule_item_id);

    for schedule_item_id in schedule_ids {
        println!("\nid: {schedule_item_id}");

        std::thread::sleep(Duration::from_secs(1));
        let endpoint = api::LessonScheduleItems {
            schedule_item_id,
            student_id,
        };
        let response = client.trigger_endpoint(endpoint).await?;
        dbg!(&response);
        println!("{response:#?}");
    }

    // println!("{}", serde_json::to_string_pretty(&response).unwrap());
    // println!("{response:#?}");

    Ok(())
}
