use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc,
};

use anyhow::{Context, Result};

use crate::{time::*, types::schedule::LessonSchedule};

mod fetcher;

pub struct State {
    db: crate::database::Database,
    student_id: u64,
    high_priority_queue: mpsc::Sender<Query>,
    low_priority_queue: mpsc::Sender<Query>,
    updated_since_last_reload: &'static AtomicBool,
}
pub enum Query {
    UpdateSchedule,
    SaturateLessonSchedule { schedule_item_id: u64 },
}
pub enum StateUpdate {}

impl State {
    pub fn open(api_key: String, student_id: u64) -> Result<Self> {
        let (high_priority_sender, high_priority_receiver) = mpsc::channel();
        let (low_priority_sender, low_priority_receiver) = mpsc::channel();
        let updated_since_last_query = Box::leak(Box::new(AtomicBool::new(false))) as &_;
        let db = crate::database::Database::open()?;
        let db2 = crate::database::Database::open()?;
        std::thread::spawn(move || {
            fetcher::start_fetcher(
                api_key,
                student_id,
                db2,
                high_priority_receiver,
                low_priority_receiver,
                updated_since_last_query,
            )
        });
        Ok(Self {
            db,
            student_id,
            high_priority_queue: high_priority_sender,
            low_priority_queue: low_priority_sender,
            updated_since_last_reload: updated_since_last_query,
        })
    }
    pub fn schedule(&self, from: Date, to: Date) -> Result<Vec<LessonSchedule>> {
        let result = self
            .db
            .connection
            .prepare_cached(
                "
                SELECT *
                FROM lesson_schedules
                WHERE date >= ?1 AND date <= ?2
                ",
            )
            .context("error in sql expression that queries schedule")?
            .query_map((from, to), |row| {
                Ok(LessonSchedule {
                    subject_name: row.get_unwrap(0),
                    room_number: row.get_unwrap(1),
                    date: row.get_unwrap(2),
                    begin_time: row.get_unwrap(3),
                    end_time: row.get_unwrap(4),
                    absence_reason_id: row.get_unwrap(5),
                    schedule_item_id: row.get_unwrap(6),
                    lesson_type: row.get_unwrap(7),
                })
            })
            .context("failed to query schedule")?
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        self.high_priority_queue
            .send(Query::UpdateSchedule)
            .unwrap();
        for schedule_item_id in result.iter().map(|lesson| lesson.schedule_item_id) {
            self.low_priority_queue
                .send(Query::SaturateLessonSchedule { schedule_item_id })
                .unwrap();
        }
        Ok(result)
    }
    pub fn should_update(&self) -> bool {
        let should_update = self.updated_since_last_reload.load(Ordering::SeqCst);
        self.updated_since_last_reload
            .store(false, Ordering::SeqCst);
        should_update
    }
}
