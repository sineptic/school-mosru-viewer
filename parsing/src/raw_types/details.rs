use serde::Deserialize;

use crate::{
    raw_types::{enums::*, homework::AdditionalMaterial},
    time,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonDetails {
    pub id: u64,
    pub teacher: Teacher,
    pub plan_id: u64,
    pub date: time::Date,
    pub begin_time: time::Time,
    pub end_time: time::Time,
    pub room_number: String,
    pub lesson_homeworks: Vec<LessonHomework>,

    pub begin_utc: u64,
    pub end_utc: u64,
    pub subject_id: u64,
    pub subject_name: String,
    pub course_lesson_type: Option<CourseLessonType>,
    pub room_name: String,
    pub building_name: String,
    pub is_missed_lesson: bool,
    pub lesson_type: LessonType,
    pub homework_to_give: Option<Vec<HomeworkToGive>>,
    pub details: Details,
    pub remote_lesson: Option<RemoteLessonInfo>,
    pub lesson_education_type: LessonEducationType,
    pub disease_status_type: Option<DeseaseStatusType>,
    pub is_virtual: bool,
    pub homework_presence_status_id: u64,

    pub marks: Vec<()>,
    pub created_date_time: (),
    pub field_name: (),
    pub comment: (),
    pub esz_field_id: (),
    pub teacher_comments: Vec<()>,
    pub lesson_type_nsi: (),
    pub control: (),
    pub evaluation: (),
    pub theme_mastery: (),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoteLessonInfo {
    link_to_join: String,
    _link_to_record: (),
    record_preview: (),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Teacher {
    pub last_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub birth_date: (),
    pub sex: (),
    pub user_id: (),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonHomework {
    pub homework: String,
    pub homework_entry_student_id: u64,
    pub homework_id: u64,
    pub homework_entry_id: u64,
    pub homework_created_at: String,
    pub homework_updated_at: String,
    pub is_done: bool,
    pub additional_materials: Vec<AdditionalMaterial>,
    pub date_assigned_on: String,
    pub date_prepared_for: String,

    pub is_smart: bool,
    pub attachments: Vec<()>,
    pub written_answer: (),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HomeworkToGive {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Details {
    pub content: Vec<()>,
    pub theme: Option<Theme>,
    #[serde(rename = "lessonId")]
    pub lesson_id: u64,
    pub lesson_topic: String,
    pub additional_materials: Vec<AdditionalMaterial>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub id: Option<u64>,
    pub title: Option<String>,
    #[serde(rename = "themeIntegrationId")]
    pub theme_integration_id: Option<u64>,
    pub average_mark: (),
    pub theme_frames: Vec<Theme>,
    pub oge_task_name: (),
    pub ege_task_name: Option<Vec<String>>,
}
