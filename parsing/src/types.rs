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
    use serde::Serialize;

    use crate::{raw_types::schedule as raw_types, time};

    #[derive(Debug, Serialize)]
    pub struct LessonSchedule {
        pub id: u64,
        pub subject_id: u64,
        pub date: time::Date,
        pub begin_time: time::Time,
        pub end_time: time::Time,
        pub bell_id: u64,
        pub group_id: u64,
        pub absence_reason_id: Option<u64>,
        pub schedule_item_id: u64,
        pub is_virtual: bool,
    }
    impl LessonSchedule {
        fn from(value: raw_types::Lesson, date: time::Date) -> Option<Self> {
            // panics needed to determine what data is useful.
            assert!(!value.is_virtual);
            Some(LessonSchedule {
                id: value.lesson_id?,
                subject_id: value.subject_id.unwrap(),
                date,
                begin_time: value.begin_time,
                end_time: value.end_time,
                bell_id: value.bell_id.unwrap(),
                group_id: value.group_id,
                absence_reason_id: value.absence_reason_id,
                schedule_item_id: value.schedule_item_id,
                is_virtual: value.is_virtual,
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
