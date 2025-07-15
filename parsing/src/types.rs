pub mod marks {
    use serde::Serialize;

    use crate::{raw_types::marks as raw_types, time};

    type MarkValue = u8;
    type MarkId = u64;
    type SubjectId = u64;
    type YearDate = String;

    #[derive(Debug, Serialize)]
    pub struct Mark {
        pub id: MarkId,
        pub subject_id: SubjectId,
        pub value: MarkValue,
        pub weight: u8,
        pub control_form_name: String,
        pub comment: Option<String>,
        pub point_date: Option<time::Date>,
        pub date: time::Date,
        pub is_point: bool,
        pub is_exam: bool,
    }
    #[derive(Debug, Serialize)]
    pub struct Period {
        pub start: YearDate,
        pub end: YearDate,
        pub period_mark: Option<MarkValue>,
    }
    #[derive(Debug, Serialize)]
    pub struct Subject {
        pub id: SubjectId,
        pub name: String,
        pub year_mark: Option<MarkValue>,
        pub periods: Vec<Period>,
    }
    pub fn extract_subject_info(subject_raw: raw_types::SubjectMarks) -> (Subject, Vec<Mark>) {
        let year_mark = subject_raw.year_mark.map(|x| x.parse().unwrap());
        let subject_id = subject_raw.subject_id;
        let mut marks = Vec::new();
        let mut periods = Vec::new();
        for period in subject_raw.periods {
            for mark in period.marks {
                marks.push(Mark {
                    id: mark.id,
                    subject_id,
                    value: mark.value.parse().unwrap(),
                    comment: mark
                        .comment
                        .map(|x| x.trim().to_owned())
                        .and_then(|x| if x.is_empty() { None } else { Some(x) }),
                    weight: mark.weight,
                    point_date: mark.point_date,
                    control_form_name: mark.control_form_name,
                    date: mark.date,
                    is_point: mark.is_point,
                    is_exam: mark.is_exam,
                });
            }
            periods.push(Period {
                start: period.start,
                end: period.end,
                period_mark: period.fixed_value.map(|x| x.parse().unwrap()),
            });
        }

        (
            Subject {
                id: subject_id,
                name: subject_raw.subject_name,
                year_mark,
                periods,
            },
            marks,
        )
    }
    pub fn extract_marks_info(marks_raw: raw_types::Marks) -> (Vec<Subject>, Vec<Mark>) {
        let (subjects, marks): (_, Vec<Vec<Mark>>) = marks_raw
            .payload
            .into_iter()
            .map(extract_subject_info)
            .unzip();
        (subjects, marks.into_iter().flatten().collect())
    }
}

pub mod schedule {
    use serde::{Deserialize, Serialize};

    use crate::{
        raw_types::{enums::LessonEducationType, schedule as raw_types},
        time,
    };

    type Loading<T> = Option<T>;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LessonSchedule {
        pub subject_name: String,
        pub room_number: Loading<u32>,
        pub date: time::Date,
        pub begin_time: time::Time,
        pub end_time: time::Time,
        pub absence_reason_id: Option<u64>,
        pub schedule_item_id: u64,
        pub lesson_type: LessonEducationType,
    }
    impl LessonSchedule {
        fn from(value: raw_types::Lesson, date: time::Date) -> Option<Self> {
            assert!(!value.is_virtual);

            Some(LessonSchedule {
                subject_name: value.subject_name.unwrap_or(value.group_name),
                room_number: None,
                date,
                begin_time: value.begin_time,
                end_time: value.end_time,
                absence_reason_id: value.absence_reason_id,
                schedule_item_id: value.schedule_item_id,
                lesson_type: value.lesson_education_type,
            })
        }
    }
    pub fn transform(raw_schedule: raw_types::DaySchedule) -> Vec<LessonSchedule> {
        let date = raw_schedule.date;
        raw_schedule
            .lessons
            .into_iter()
            .filter_map(|l| LessonSchedule::from(l, date))
            .collect()
    }
}

pub mod homework {
    use serde::{Deserialize, Serialize};

    use crate::{
        raw_types::{self},
        time::*,
    };

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Homework {
        pub id: u64,
        pub task: String,
        pub subject_name: String,
        pub created_at: DateTime,
        pub updated_at: DateTime,
        pub assigned_on: Date,
        pub date_prepared_for: Date,
        pub additional_materials: Vec<AdditionalMaterial>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AdditionalMaterial {
        pub id: String,
        pub title: Option<String>,
        pub urls: Vec<String>,
    }

    impl From<raw_types::homework::AdditionalMaterial> for AdditionalMaterial {
        fn from(value: raw_types::homework::AdditionalMaterial) -> Self {
            assert!(
                value.uuid.is_some() && value.id.is_none()
                    || value.uuid.is_none() && value.id.is_some()
            );

            Self {
                id: value.uuid.unwrap_or_else(|| value.id.unwrap().to_string()),
                title: value.title,
                urls: value.urls.into_iter().map(|x| x.url).collect(),
            }
        }
    }
    impl From<raw_types::homework::Homework> for Homework {
        fn from(value: raw_types::homework::Homework) -> Self {
            assert_eq!(value.homework, value.description);
            assert_eq!(
                value.date_prepared_for.time,
                Time {
                    hours: 0,
                    minutes: 0
                }
            );

            Self {
                id: value.homework_id,
                task: value.homework,
                subject_name: value.subject_name,
                additional_materials: value.materials.into_iter().map(|x| x.into()).collect(),
                created_at: value.homework_created_at,
                updated_at: value.homework_updated_at,
                assigned_on: value.date_assigned_on,
                date_prepared_for: value.date_prepared_for.date,
            }
        }
    }
}
