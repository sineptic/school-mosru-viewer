use std::cmp::min;

use reqwest::{Client, Method, Url, header::HeaderMap};
use serde::de::DeserializeOwned;

use crate::{raw_types, time};

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
        let url = endpoint.url();
        let response = self.client.request(E::METHOD, url.clone()).send().await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            panic!(
                "Endpoint at url '{url}' return this error: '{text}' with this status code '{status}'",
            )
        }
        let response = response.text().await?;
        match serde_json::from_str(&response) {
            Ok(response) => Ok(E::transform_response(response)),
            Err(json_err) => {
                eprintln!("json decode error: {json_err}");
                let line = json_err.line();
                let column = json_err.column();
                assert!(line == 1);
                let start = column.saturating_sub(20);
                let end = min(column + 20, response.len());
                let span = &response[start..end];
                eprintln!();
                eprintln!("context: '{span}'");
                eprintln!("{}^", " ".repeat(29));

                panic!();
            }
        }
        // let response = response.json::<E::RawResponse>().await.map_err(|err| {
        //     let Some(source) = err.source() else {
        //         return anyhow::anyhow!("{err}");
        //     };
        //     let Some(json_err): Option<&serde_json::Error> = source.downcast_ref() else {
        //         return anyhow::anyhow!("{err}");
        //     };
        //     let line = json_err.line();
        //     let column = json_err.column();
        //     let reason = json_err.to_string();
        //     response;

        //     todo!();
        // })?;
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
    pub dates: Vec<time::Date>,
}
impl From<Schedule> for Url {
    fn from(value: Schedule) -> Self {
        assert!(!value.dates.is_empty());
        format!(
            "https://school.mos.ru/api/family/web/v1/schedule/short?student_id={}&dates={}",
            value.student_id,
            value
                .dates
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("%2C")
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
impl Schedule {
    pub fn new<D>(student_id: u64, dates: impl IntoIterator<Item = D>) -> Self
    where
        D: AsRef<str>,
    {
        Self {
            student_id,
            dates: dates
                .into_iter()
                .map(|d| d.as_ref().parse().unwrap())
                .collect(),
        }
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

pub struct LessonScheduleItems {
    pub schedule_item_id: u64,
    pub student_id: u64,
}
impl From<LessonScheduleItems> for Url {
    fn from(value: LessonScheduleItems) -> Self {
        format!(
            "https://school.mos.ru/api/family/web/v1/lesson_schedule_items/531559037?student_id={}",
            value.student_id
        )
        .parse()
        .unwrap()
    }
}
impl ApiEndpoint for LessonScheduleItems {
    const METHOD: Method = Method::GET;

    type RawResponse = raw_types::details::LessonDetails;

    type ProcessedResponse = raw_types::details::LessonDetails;

    fn transform_response(raw_response: Self::RawResponse) -> Self::ProcessedResponse {
        raw_response
    }
}
