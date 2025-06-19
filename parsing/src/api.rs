use reqwest::Url;

pub trait ApiEndpoint: Into<Url> {
    fn url(self) -> Url {
        self.into()
    }
}

pub struct Marks {
    pub student_id: u32,
}
impl From<Marks> for Url {
    fn from(this: Marks) -> Self {
        format!(
            "https://school.mos.ru/api/family/web/v1/subject_marks?student_id={}",
            this.student_id
        )
        .parse()
        .unwrap()
    }
}
impl ApiEndpoint for Marks {}
