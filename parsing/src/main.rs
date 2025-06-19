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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = api_reqwest_client(std::env::var("MOSRU_BEARER").unwrap());

    let student_id = 31823383;
    let response = client.get(api::Marks { student_id }.url()).send().await?;
    println!("{}", response.text().await?);

    Ok(())
}
