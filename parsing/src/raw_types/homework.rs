use serde::Deserialize;

use crate::{raw_types::enums::*, time::*};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    pub payload: Vec<Homework>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Homework {
    pub homework: String,
    pub subject_name: String,
    pub homework_entry_student_id: u64,
    pub homework_id: u64,
    pub homework_entry_id: u64,
    pub homework_created_at: DateTime,
    pub homework_updated_at: DateTime,
    pub is_done: bool,
    pub materials: Vec<AdditionalMaterial>,
    pub date_assigned_on: Date,
    pub date_prepared_for: DateTime,

    #[serde(rename = "type")]
    pub type_field: HomeworkType,
    pub lesson_date_time: DateTime,
    pub subject_id: u64,
    pub group_id: u64,
    pub date: Date,

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
