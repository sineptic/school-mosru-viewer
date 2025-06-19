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

mod raw_types {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Marks {
        pub payload: Vec<SubjectMarks>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct SubjectMarks {
        pub average: String,
        pub dynamic: String,
        pub periods: Vec<PeriodMarks>,
        pub subject_name: String,
        pub subject_id: u32,
        pub average_by_all: String,
        pub year_mark: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PeriodMarks {
        pub start: String,
        pub end: String,
        pub title: String,
        pub dynamic: String,
        pub value: String,
        pub marks: Vec<Mark>,
        pub count: usize,
        pub target: Option<String>,
        pub fixed_value: String,
        pub start_iso: String,
        pub end_iso: String,
    }
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct Mark {
        pub id: u64,
        pub value: String,
        pub values: (),
        pub comment: Option<String>,
        pub weight: u32,
        pub point_date: Option<String>,
        pub control_form_name: String,
        pub comment_exists: bool,
        pub created_at: (),
        pub updated_at: (),
        pub criteria: (),
        pub date: String,
        pub is_point: bool,
        pub is_exam: bool,
        pub original_grade_system_type: String,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let client = api_reqwest_client(std::env::var("MOSRU_BEARER").unwrap());

    let student_id = 31823383;
    let response = client.get(api::Marks { student_id }.url()).send().await?;
    println!("{:#?}", response.json::<raw_types::Marks>().await?);

    Ok(())
}
