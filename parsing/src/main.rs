use api::ApiClient;

mod api;
mod raw_types;
mod time;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = ApiClient::new(std::env::var("MOSRU_BEARER").unwrap());

    let student_id = 31823383;
    let endpoint = api::LessonScheduleItems {
        student_id,
        schedule_item_id: 531559037,
    };
    let response = client.trigger_endpoint(endpoint).await?;

    // println!("{}", serde_json::to_string_pretty(&response).unwrap());
    println!("{response:#?}");

    Ok(())
}
