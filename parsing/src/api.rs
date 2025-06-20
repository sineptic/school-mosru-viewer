use reqwest::{Client, Method, Url, header::HeaderMap};
use serde::de::DeserializeOwned;

pub struct ApiClient {
    client: Client,
}
impl ApiClient {
    pub fn new(authorization_token: impl AsRef<str>) -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
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
                .unwrap(),
        }
    }
    pub async fn trigger_endpoint<E: ApiEndpoint>(
        &self,
        endpoint: E,
    ) -> anyhow::Result<E::ProcessedResponse> {
        let response = self
            .client
            .request(E::METHOD, endpoint.url())
            .send()
            .await?
            .json::<E::RawResponse>()
            .await?;
        Ok(E::transform_response(response))
    }
}

pub trait ApiEndpoint: Into<Url> {
    fn url(self) -> Url {
        self.into()
    }
    const METHOD: Method;

    type RawResponse: DeserializeOwned;
    type ProcessedResponse;
    fn transform_response(raw_response: Self::RawResponse) -> Self::ProcessedResponse;
}

pub struct Marks {
    pub student_id: u64,
}
impl From<Marks> for Url {
    fn from(value: Marks) -> Self {
        format!(
            "https://school.mos.ru/api/family/web/v1/subject_marks?student_id={}",
            value.student_id
        )
        .parse()
        .unwrap()
    }
}
impl ApiEndpoint for Marks {
    const METHOD: Method = Method::GET;
    type RawResponse = crate::raw_types::marks::Marks;
    type ProcessedResponse = (
        Vec<crate::types::marks::Subject>,
        Vec<crate::types::marks::Mark>,
    );
    fn transform_response(raw_response: Self::RawResponse) -> Self::ProcessedResponse {
        crate::types::marks::extract_marks_info(raw_response)
    }
}

pub struct Schedule {
    pub student_id: u64,
    pub dates: Vec<String>,
}
impl From<Schedule> for Url {
    fn from(value: Schedule) -> Self {
        assert!(!value.dates.is_empty());
        format!(
            "https://school.mos.ru/api/family/web/v1/schedule/short?student_id={}&dates={}",
            value.student_id,
            value.dates.join("%2C")
        )
        .parse()
        .unwrap()
    }
}
impl ApiEndpoint for Schedule {
    const METHOD: Method = Method::GET;
    type RawResponse = crate::raw_types::schedule::Schedule;
    type ProcessedResponse = Vec<crate::types::schedule::LessonSchedule>;
    fn transform_response(raw_response: Self::RawResponse) -> Self::ProcessedResponse {
        raw_response
            .payload
            .into_iter()
            .flat_map(crate::types::schedule::transform)
            .collect()
    }
}

// pub struct Homework {
//     pub student_id: u64,
//     pub from: String,
//     pub to: String,
// }
// impl From<Homework> for Url {
//     fn from(value: Homework) -> Self {
//         format!(
//             "https://school.mos.ru/api/family/web/v1/homeworks?from={}&to={}&student_id={}",
//             value.from, value.to, value.student_id,
//         )
//         .parse()
//         .unwrap()
//     }
// }
// impl ApiEndpoint for Homework {}
