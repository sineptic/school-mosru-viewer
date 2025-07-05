#[allow(unused)]
pub mod marks {
    use serde::Deserialize;

    use crate::time;

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
        pub subject_id: u64,
        pub average_by_all: String,
        pub year_mark: Option<String>,
    }
    type YearDate = String;
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct PeriodMarks {
        pub start: YearDate,
        pub end: YearDate,
        pub title: String,
        pub dynamic: String,
        pub value: String,
        pub marks: Vec<Mark>,
        pub count: usize,
        pub target: Option<String>,
        pub fixed_value: Option<String>,
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
        pub weight: u8,
        pub point_date: Option<time::Date>,
        pub control_form_name: String,
        pub comment_exists: bool,
        pub created_at: (),
        pub updated_at: (),
        pub criteria: (),
        pub date: time::Date,
        pub is_point: bool,
        pub is_exam: bool,
        pub original_grade_system_type: String,
    }
}

#[allow(unused)]
pub mod schedule {
    use serde::{Deserialize, Serialize};

    use crate::time;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Schedule {
        pub payload: Vec<DaySchedule>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct DaySchedule {
        pub date: time::Date,
        pub lessons: Vec<Lesson>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Lesson {
        pub lesson_id: Option<u64>,
        pub begin_time: time::Time,
        pub end_time: time::Time,
        pub bell_id: Option<u64>,
        pub subject_name: Option<String>,
        pub lesson_type: String,
        pub group_id: u64,
        pub group_name: String,
        pub lesson_education_type: String,
        pub evaluation: (),
        pub absence_reason_id: Option<u64>,
        pub subject_id: Option<u64>,
        pub lesson_name: Option<String>,
        pub schedule_item_id: u64,
        pub is_virtual: bool,
    }
}

#[allow(unused)]
pub mod homework {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Root {
        pub payload: Vec<Payload>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Payload {
        #[serde(rename = "type")]
        pub type_field: String,
        pub description: String,
        pub comments: Vec<()>,
        pub materials: Vec<AdditionalMaterial>,
        pub homework: String,
        pub homework_entry_student_id: u64,
        pub attachments: Vec<()>,
        pub subject_id: u64,
        pub group_id: u64,
        pub date: String,
        pub date_assigned_on: String,
        pub subject_name: String,
        pub lesson_date_time: String,
        pub is_done: bool,
        pub has_teacher_answer: bool,
        pub homework_id: u64,
        pub homework_entry_id: u64,
        pub homework_created_at: String,
        pub homework_updated_at: String,
        pub written_answer: (),
        pub date_prepared_for: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct AdditionalMaterial {
        pub uuid: Option<String>,
        #[serde(rename = "type")]
        pub type_field: Option<String>,
        pub selected_mode: Option<String>,
        pub type_name: String,
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
        pub type_field: String,
    }
}

#[allow(unused)]
pub mod details {
    use serde::Deserialize;

    use crate::raw_types::homework::AdditionalMaterial;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct LessonDetails {
        pub id: u64,
        pub plan_id: u64,
        pub date: String,
        pub begin_time: String,
        pub begin_utc: u64,
        pub end_time: String,
        pub end_utc: u64,
        pub subject_id: u64,
        pub subject_name: String,
        pub teacher: Teacher,
        pub course_lesson_type: (),
        pub room_number: String,
        pub room_name: String,
        pub building_name: String,
        pub marks: Vec<()>,
        pub created_date_time: (),
        pub is_missed_lesson: bool,
        pub lesson_type: String,
        pub field_name: (),
        pub comment: (),
        pub lesson_homeworks: Vec<LessonHomework>,
        pub homework_to_give: Vec<HomeworkToGive>,
        pub details: Details,
        pub esz_field_id: (),
        pub teacher_comments: Vec<()>,
        pub lesson_type_nsi: (),
        pub remote_lesson: (),
        pub control: (),
        pub evaluation: (),
        pub lesson_education_type: String,
        pub disease_status_type: (),
        pub theme_mastery: (),
        pub is_virtual: bool,
        pub homework_presence_status_id: u64,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Teacher {
        pub last_name: String,
        pub first_name: String,
        pub middle_name: String,
        pub birth_date: (),
        pub sex: (),
        pub user_id: (),
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct LessonHomework {
        pub homework: String,
        pub homework_entry_student_id: i64,
        pub homework_id: i64,
        pub homework_entry_id: i64,
        pub attachments: Vec<()>,
        pub homework_created_at: String,
        pub homework_updated_at: String,
        pub is_done: bool,
        pub is_smart: bool,
        pub additional_materials: Vec<AdditionalMaterial>,
        pub written_answer: (),
        pub date_assigned_on: String,
        pub date_prepared_for: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct HomeworkToGive {
        pub id: i64,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Details {
        pub content: Vec<()>,
        pub theme: Theme,
        #[serde(rename = "lessonId")]
        pub lesson_id: i64,
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
}
