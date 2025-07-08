use serde::Deserialize;

use crate::{raw_types::enums::*, time};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub payload: Vec<Payload>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Payload {
    pub homework: String,
    pub homework_entry_student_id: u64,
    pub homework_id: u64,
    pub homework_entry_id: u64,
    pub homework_created_at: String,
    pub homework_updated_at: String,
    pub is_done: bool,
    pub materials: Vec<AdditionalMaterial>,
    pub date_assigned_on: time::Date,
    pub date_prepared_for: String,

    #[serde(rename = "type")]
    pub type_field: HomeworkType,
    pub subject_name: String,
    pub lesson_date_time: String,
    pub subject_id: u64,
    pub group_id: u64,
    pub date: time::Date,

    /// Always equals `homework` field
    pub description: String,
    pub has_teacher_answer: bool,
    pub attachments: Vec<()>,
    pub written_answer: (),
    pub comments: Vec<()>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdditionalMaterial {
    pub uuid: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<AdditionalMaterialType>,
    pub selected_mode: Option<AdditionalMaterialSelectedMode>,
    pub type_name: AdditionalMaterialName,
    pub id: Option<u64>,
    pub urls: Vec<Url>,
    pub description: (),
    pub content_type: (),
    pub title: Option<String>,
    pub action_id: u64,
    pub action_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Url {
    pub url: String,
    #[serde(rename = "type")]
    pub type_field: UrlType,
}
