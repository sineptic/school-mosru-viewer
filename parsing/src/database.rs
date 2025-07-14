use anyhow::Context;
use rusqlite::{Connection, OptionalExtension, Transaction};
use serde::{Serialize, de::DeserializeOwned};

use crate::types::{
    self,
    homework::{AdditionalMaterial, Homework},
};
pub struct Database {
    pub connection: Connection,
}
pub struct MutDatabase<'a> {
    transaction: Transaction<'a>,
}
impl Database {
    pub fn open() -> Result<Self> {
        let connection = Connection::open("db.sqlite")?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS additional_homework_materials (
                    id    TEXT PRIMARY KEY,
                    title TEXT,
                    urls  TEXT NOT NULL
                )",
            (),
        )?;
        connection.execute(
            "CREATE TABLE IF NOT EXISTS homeworks (
                    id                   INTEGER PRIMARY KEY,
                    task                 TEXT NOT NULL,
                    entry_id             INTEGER NOT NULL,
                    entry_student_id     INTEGER NOT NULL,
                    created_at           TEXT NOT NULL,
                    updated_at           TEXT NOT NULL,
                    assigned_on          TEXT NOT NULL,
                    date_prepared_for    TEXT NOT NULL,
                    additional_materials TEXT NOT NULL
                )",
            (),
        )?;
        Ok(Self { connection })
    }
    pub fn transaction(&mut self) -> Result<MutDatabase<'_>> {
        Ok(MutDatabase {
            transaction: self
                .connection
                .transaction()
                .context("Failed to start transaction")?,
        })
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
        let mut stmt = self.transaction.prepare_cached(
            "INSERT OR REPLACE INTO additional_homework_materials (id, title, urls)
                VALUES (?1, ?2, ?3)",
        )?;
        for material in materials {
            stmt.execute((material.id, material.title, serialize(material.urls)))
                .context("Failed to store homework additional material")?;
        }
        Ok(())
    }

    pub fn store_homeworks(&self, hws: Vec<Homework>) -> Result<()> {
        let mut stmt = self.transaction.prepare_cached(
            "INSERT OR REPLACE INTO homeworks (id, task, entry_id, entry_student_id, created_at, updated_at, assigned_on, date_prepared_for, additional_materials)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    )?;

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
                        hw.entry_id,
                        hw.entry_student_id,
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
        )?;

        for hw in other {
            stmt.execute(hw).context("Failed to store homework")?;
        }
        Ok(())
    }
}

impl Database {
    pub fn get_homework_additional_material(
        &self,
        id: &str,
    ) -> Result<Option<types::homework::AdditionalMaterial>> {
        let mut stmt = self
            .connection
            .prepare("SELECT (id, title, urls) FROM additional_homework_materials WHERE id = ?1")?;

        stmt.query_one([id], |x| {
            Ok(AdditionalMaterial {
                id: x.get_unwrap(0),
                title: x.get_unwrap(1),
                urls: {
                    let string: String = x.get_unwrap(2);
                    deserialize(string)
                },
            })
        })
        .optional()
        .context("Failed to get homework additional material")
    }
    pub fn get_homework(&self, id: u64) -> Result<Option<Homework>> {
        let mut stmt = self.connection.prepare(
            "SELECT (id, task, entry_id, entry_student_id, created_at, updated_at, assigned_on, date_prepared_for, additional_materials) FROM homeworks WHERE id = ?1",
        )?;

        let a = stmt
            .query_one([id], |x| {
                Ok((
                    x.get_unwrap(0),
                    x.get_unwrap(1),
                    x.get_unwrap(2),
                    x.get_unwrap(3),
                    x.get_unwrap(4),
                    x.get_unwrap(5),
                    x.get_unwrap(6),
                    x.get_unwrap(7),
                    deserialize::<Vec<String>>(x.get_unwrap(8)),
                ))
            })
            .optional()
            .context("Failed to store homework")?;
        if let Some(a) = a {
            Ok(Some(Homework {
                id: a.0,
                task: a.1,
                entry_id: a.2,
                entry_student_id: a.3,
                created_at: a.4,
                updated_at: a.5,
                assigned_on: a.6,
                date_prepared_for: a.7,
                additional_materials: {
                    a.8.into_iter()
                        .map(|id| {
                            self.get_homework_additional_material(&id)
                                .transpose()
                                .expect("All homework's additional materials should be added to db")
                        })
                        .collect::<Result<Vec<_>, _>>()?
                },
            }))
        } else {
            Ok(None)
        }
    }
}
