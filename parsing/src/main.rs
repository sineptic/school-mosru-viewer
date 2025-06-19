use reqwest::header::HeaderMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let student_id = 31823383;
    let url =
        format!("https://school.mos.ru/api/family/web/v1/subject_marks?student_id={student_id}");
    // let response = reqwest::get(url).await;
    let authorization = std::env::var("MOSRU_BEARER").unwrap();
    let client = reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter([
            (
                reqwest::header::AUTHORIZATION,
                format!("Bearer {authorization}").parse().unwrap(),
            ),
            (
                "x-mes-subsystem".parse().unwrap(),
                "familyweb".parse().unwrap(),
            ),
        ]))
        .build()
        .unwrap();

    let request = client.get(url).build().unwrap();
    let response = client.execute(request).await?;
    println!("{}", response.text().await?);

    Ok(())
}
