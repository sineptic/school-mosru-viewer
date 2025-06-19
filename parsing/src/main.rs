use api::ApiEndpoint;
use reqwest::header::HeaderMap;
use types::marks::extract_marks_info;

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
        .get(api::Marks { student_id }.url())
        .send()
        .await?
        .json::<raw_types::marks::Marks>()
        .await?;
    let (subjects, marks) = extract_marks_info(response);
    println!("{{");
    println!(
        "subjects: {},",
        serde_json::to_string_pretty(&subjects).unwrap()
    );
    println!("marks: {}", serde_json::to_string_pretty(&marks).unwrap());
    println!("}}");

    Ok(())
}
