use api::ApiEndpoint;
use reqwest::header::HeaderMap;

mod api;

fn api_reqwest_client(authorization_token: impl AsRef<str>) -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter([
            (
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", authorization_token.as_ref())
                    .parse()
                    .unwrap(),
            ),
            (
                "x-mes-subsystem".parse().unwrap(),
                "familyweb".parse().unwrap(),
            ),
        ]))
        .build()
        .unwrap()
}

mod raw_types;
mod types;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = api_reqwest_client(std::env::var("MOSRU_BEARER").unwrap());

    let student_id = 31823383;
    let response = client
        .get(
            api::Schedule {
                student_id,
                dates: vec![
                    "2024-09-02".into(),
                    "2024-09-03".into(),
                    "2024-09-04".into(),
                    "2024-09-05".into(),
                    "2024-09-06".into(),
                    "2024-09-07".into(),
                    "2024-09-08".into(),
                ],
            }
            .url(),
        )
        .send()
        .await?
        .json::<raw_types::schedule::Schedule>()
        .await?;
    let lessons = response
        .payload
        .into_iter()
        .flat_map(types::schedule::transform)
        .collect::<Vec<_>>();
    println!("{}", serde_json::to_string_pretty(&lessons).unwrap());

    Ok(())
}
