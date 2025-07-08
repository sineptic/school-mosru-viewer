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
