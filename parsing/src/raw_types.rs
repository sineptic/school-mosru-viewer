#[allow(unused)]
pub mod marks {
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
        pub fixed_value: Option<String>,
        pub start_iso: String,
        pub end_iso: String,
    }
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    #[allow(unused)]
    pub struct Mark {
        pub id: u64,
        pub value: String,
        pub values: (),
        pub comment: Option<String>,
        pub weight: u8,
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

#[allow(unused)]
pub mod schedule {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Deserialize)]
    pub struct Schedule {
        pub payload: Vec<DaySchedule>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct DaySchedule {
        pub date: String,
        pub lessons: Vec<Lesson>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Lesson {
        pub lesson_id: Option<u64>,
        pub begin_time: String,
        pub end_time: String,
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
