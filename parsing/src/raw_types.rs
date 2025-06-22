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
        pub type_field: String,
        pub selected_mode: Option<String>,
        pub type_name: String,
        pub id: Option<u64>,
        pub urls: Vec<Url>,
        pub description: (),
        pub content_type: (),
        pub title: String,
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
