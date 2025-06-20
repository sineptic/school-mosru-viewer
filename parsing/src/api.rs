use reqwest::Url;

pub trait ApiEndpoint: Into<Url> {
    fn url(self) -> Url {
        self.into()
    }
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
impl ApiEndpoint for Marks {}

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
impl ApiEndpoint for Schedule {}
