use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Receiver,
    },
    time::{Duration, Instant},
};

use crate::{api, database::Database, localfirst::Query, raw_types::enums::LessonEducationType};

#[derive(Default)]
struct Cache {
    full_schedule_fetch: Option<Instant>,
    detailed_schedule_fetch: HashMap<u64, Instant>,
}

const INVALIDATION_TIME: Duration = Duration::from_secs(600);
const REQUESTS_DELAY: Duration = Duration::from_millis(100);

pub fn start_fetcher(
    api_key: String,
    student_id: u64,
    mut db: Database,
    high_priority_queue: Receiver<Query>,
    low_priority_queue: Receiver<Query>,
    updated_since_last_query: &'static AtomicBool,
) {
    let api_client = crate::api::ApiClient::new(api_key);
    let mut cache = Cache::default();
    let mut last_request = Instant::now();
    loop {
        sleep_until(last_request);
        let Ok(query) = high_priority_queue
            .try_recv()
            .or_else(|_| low_priority_queue.try_recv())
            .or_else(|_| high_priority_queue.recv())
        else {
            return;
        };
        let request_start = Instant::now();
        match query {
            Query::UpdateSchedule => {
                if cache.schedule_update_cached() {
                    log::debug!("schedule update cached");
                    continue;
                }
                log::debug!("updating schedule");
                let schedule = api_client
                    .trigger_endpoint(api::Schedule {
                        student_id,
                        dates: crate::_all_possible_dates(),
                    })
                    .unwrap();
                db.transaction(|tr| {
                    tr.store_lesson_schedules(schedule)?;
                    Ok(())
                })
                .unwrap();
                cache.full_schedule_fetch();
            }
            Query::SaturateLessonSchedule { schedule_item_id } => {
                if cache.detailed_schedule_cached(schedule_item_id)
                    || db
                        .connection
                        .query_one::<Option<usize>, _, _>(
                            "
                            SELECT room_number
                            FROM lesson_schedules
                            WHERE schedule_item_id = ?1
                            ",
                            (schedule_item_id,),
                            |row| row.get(0),
                        )
                        .unwrap()
                        .is_some()
                {
                    log::debug!("detailed schedule update with id {schedule_item_id} cached");
                    continue;
                }
                log::debug!("updating detailed schedule with id {schedule_item_id}");
                let lesson_type: LessonEducationType = db
                    .connection
                    .query_one(
                        "
                        SELECT lesson_type
                        FROM lesson_schedules
                        WHERE schedule_item_id = ?1
                        ",
                        (schedule_item_id,),
                        |row| row.get(0),
                    )
                    .unwrap();
                let detailed_schedule = api_client
                    .trigger_endpoint(api::LessonScheduleItems {
                        student_id,
                        lesson_type,
                        schedule_item_id,
                    })
                    .unwrap();
                db.transaction(|tr| {
                    tr.add_room_for_lesson_schedule(
                        schedule_item_id,
                        detailed_schedule.room_number.parse().unwrap(),
                    )?;
                    Ok(())
                })
                .unwrap();
                cache.detailed_schedule_fetch(schedule_item_id);
            }
        }
        updated_since_last_query.store(true, Ordering::SeqCst);
        last_request = request_start;
    }
}

fn sleep_until(last_request: Instant) {
    {
        let deadline = last_request + REQUESTS_DELAY;
        let now = Instant::now();

        if let Some(delay) = deadline.checked_duration_since(now) {
            std::thread::sleep(delay);
        }
    };
}

impl Cache {
    fn schedule_update_cached(&self) -> bool {
        self.full_schedule_fetch
            .is_some_and(|time| time.elapsed() < INVALIDATION_TIME)
    }
    fn full_schedule_fetch(&mut self) {
        self.full_schedule_fetch = Some(Instant::now());
    }
    fn detailed_schedule_cached(&self, schedule_item_id: u64) -> bool {
        self.detailed_schedule_fetch
            .get(&schedule_item_id)
            .is_some_and(|time| time.elapsed() < INVALIDATION_TIME)
    }
    fn detailed_schedule_fetch(&mut self, schedule_item_id: u64) {
        self.detailed_schedule_fetch
            .insert(schedule_item_id, Instant::now());
    }
}
