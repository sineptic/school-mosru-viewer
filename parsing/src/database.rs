use anyhow::Context;
use rusqlite::{Connection, OptionalExtension, Transaction};
use serde::{Serialize, de::DeserializeOwned};

use crate::types::{
    self,
    homework::{AdditionalMaterial, Homework},
    schedule::LessonSchedule,
};
pub struct Database {
    pub connection: Connection,
}
pub struct MutDatabase<'a> {
    transaction: Transaction<'a>,
}
impl Database {
    pub fn open() -> Result<Self> {
        let file_name = "db.sqlite";
        let connection = Connection::open(file_name)
            .with_context(|| format!("Failed to open database at `{file_name}`"))?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS additional_homework_materials (
                id    TEXT PRIMARY KEY,
                title TEXT,
                urls  TEXT NOT NULL
            )",
                (),
            )
            .context("Error in SQL that creates additional_homework_materials table")?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS homeworks (
                id                   INTEGER PRIMARY KEY,
                task                 TEXT NOT NULL,
                subject_name         TEXT NOT NULL,
                created_at           TEXT NOT NULL,
                updated_at           TEXT NOT NULL,
                assigned_on          TEXT NOT NULL,
                date_prepared_for    TEXT NOT NULL,
                additional_materials TEXT NOT NULL
            )",
                (),
            )
            .context("Error in SQL that creates homeworks table")?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS lesson_schedules (
                subject_name      TEXT NOT NULL,
                room_number       INTEGER,
                date              TEXT NOT NULL,
                begin_time        TEXT NOT NULL,
                end_time          TEXT NOT NULL,
                absence_reason_id INTEGER,
                schedule_item_id  INTEGER PRIMARY KEY
            )",
                (),
            )
            .context("Error in SQL that creates lesson_schedules table")?;
        Ok(Self { connection })
    }
    pub fn start_transaction(&mut self) -> Result<MutDatabase<'_>> {
        Ok(MutDatabase {
            transaction: self
                .connection
                .transaction()
                .context("Failed to start transaction")?,
        })
    }
    pub fn transaction(
        &mut self,
        handle: impl FnOnce(&mut MutDatabase) -> anyhow::Result<()>,
    ) -> Result<()> {
        let mut tr = self
            .start_transaction()
            .context("Failed to start transaction")?;
        handle(&mut tr).context("Failed to run transaction body")?;
        tr.commit().context("Failed to commit transaction")?;
        Ok(())
    }
}
impl MutDatabase<'_> {
    pub fn commit(self) -> Result<()> {
        self.transaction
            .commit()
            .context("Failed to commit transaction")
    }
}
use anyhow::Result;

fn serialize<T: Serialize>(val: T) -> String {
    serde_json::to_string(&val).unwrap()
}
fn deserialize<T: DeserializeOwned>(input: String) -> T {
    serde_json::from_str(&input).unwrap()
}

impl MutDatabase<'_> {
    pub fn store_homework_additional_materials(
        &self,
        materials: Vec<AdditionalMaterial>,
    ) -> Result<()> {
        let mut stmt = self
            .transaction
            .prepare_cached(
                "INSERT OR IGNORE INTO additional_homework_materials (id, title, urls)
                VALUES (?1, ?2, ?3)",
            )
            .context("SQL syntax error in expression that inserts additional_homework_materials")?;
        for material in materials {
            stmt.execute((material.id, material.title, serialize(material.urls)))
                .context("Failed to store homework additional material")?;
        }
        Ok(())
    }

    pub fn store_homeworks(&self, hws: Vec<Homework>) -> Result<()> {
        let mut stmt = self.transaction.prepare_cached(
            "INSERT OR IGNORE INTO homeworks (id, task, subject_name, created_at, updated_at, assigned_on, date_prepared_for, additional_materials)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )
        .context("SQL syntax error in expression that inserts homeworks")?;

        let (additional_materials, other): (Vec<_>, Vec<_>) = hws
            .into_iter()
            .map(|hw| {
                let mut material_ids = Vec::new();
                let mut materials = Vec::new();
                for material in hw.additional_materials {
                    material_ids.push(material.id.clone());
                    materials.push(material);
                }
                (
                    materials,
                    (
                        hw.id,
                        hw.task,
                        hw.subject_name,
                        hw.created_at,
                        hw.updated_at,
                        hw.assigned_on,
                        hw.date_prepared_for,
                        serialize(material_ids),
                    ),
                )
            })
            .unzip();

        self.store_homework_additional_materials(
            additional_materials.into_iter().flatten().collect(),
        )
        .context("Failed to store additional materials for homeworks")?;

        for hw in other {
            stmt.execute(hw).context("Failed to store homework")?;
        }
        Ok(())
    }

    pub fn store_lesson_schedules(&self, lesson_schedules: Vec<LessonSchedule>) -> Result<()> {
        let mut stmt = self.transaction.prepare_cached(
            "INSERT OR IGNORE INTO lesson_schedules (subject_name, room_number, date, begin_time, end_time, absence_reason_id, schedule_item_id)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
        )
        .context("SQL syntax error in expression that inserts lesson_schedules")?;
        for lesson_schedule in lesson_schedules {
            stmt.execute((
                lesson_schedule.subject_name,
                lesson_schedule.room_number,
                lesson_schedule.date,
                lesson_schedule.begin_time,
                lesson_schedule.end_time,
                lesson_schedule.absence_reason_id,
                lesson_schedule.schedule_item_id,
            ))
            .context("Failed to add lesson schedule")?;
        }
        Ok(())
    }

    pub fn add_room_for_lesson_schedule(&self, schedule_item_id: u64, room: u32) -> Result<()> {
        let mut stmt = self
            .transaction
            .prepare_cached(
                "
                UPDATE lesson_schedules
                SET room_number = ?1
                WHERE schedule_item_id = ?2
                ",
            )
            .context("SQL syntax error in expression that adds room number to lesson schedule")?;
        stmt.execute((room, schedule_item_id))
            .context("Failed to add room number to lesson schedule")?;
        Ok(())
    }
}

// impl Database {
//     pub fn get_homework_additional_material(
//         &self,
//         id: &str,
//     ) -> Result<Option<types::homework::AdditionalMaterial>> {
//         let mut stmt = self
//             .connection
//             .prepare("SELECT (id, title, urls) FROM additional_homework_materials WHERE id = ?1")?;

//         stmt.query_one([id], |x| {
//             Ok(AdditionalMaterial {
//                 id: x.get_unwrap(0),
//                 title: x.get_unwrap(1),
//                 urls: {
//                     let string: String = x.get_unwrap(2);
//                     deserialize(string)
//                 },
//             })
//         })
//         .optional()
//         .context("Failed to get homework additional material")
//     }
//     pub fn get_homework(&self, id: u64) -> Result<Option<Homework>> {
//         let mut stmt = self.connection.prepare(
//             "SELECT (id, task, entry_id, entry_student_id, created_at, updated_at, assigned_on, date_prepared_for, additional_materials) FROM homeworks WHERE id = ?1",
//         )?;

//         let a = stmt
//             .query_one([id], |x| {
//                 Ok((
//                     x.get_unwrap(0),
//                     x.get_unwrap(1),
//                     x.get_unwrap(2),
//                     x.get_unwrap(3),
//                     x.get_unwrap(4),
//                     x.get_unwrap(5),
//                     x.get_unwrap(6),
//                     deserialize::<Vec<String>>(x.get_unwrap(7)),
//                 ))
//             })
//             .optional()
//             .context("Failed to store homework")?;
//         if let Some(a) = a {
//             Ok(Some(Homework {
//                 id: a.0,
//                 task: a.1,
//                 subject_name: a.2,
//                 created_at: a.3,
//                 updated_at: a.4,
//                 assigned_on: a.5,
//                 date_prepared_for: a.6,
//                 additional_materials: {
//                     a.7.into_iter()
//                         .map(|id| {
//                             self.get_homework_additional_material(&id)
//                                 .transpose()
//                                 .expect("All homework's additional materials should be added to db")
//                         })
//                         .collect::<Result<Vec<_>, _>>()?
//                 },
//             }))
//         } else {
//             Ok(None)
//         }
//     }
// }
