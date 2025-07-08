use serde::{Deserialize, Serialize};

use crate::{raw_types::enums::*, time};

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
    pub lesson_type: LessonType,
    pub group_id: u64,
    pub group_name: String,
    pub lesson_education_type: LessonEducationType,
    pub evaluation: (),
    pub absence_reason_id: Option<u64>,
    pub subject_id: Option<u64>,
    pub lesson_name: Option<String>,
    pub schedule_item_id: u64,
    pub is_virtual: bool,
}
