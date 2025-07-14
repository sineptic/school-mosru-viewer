use std::collections::BTreeMap;

use crate::{api::ApiClient, time::Date};

mod api;
#[allow(unused)]
mod raw_types;
mod time;
mod types;

mod database {
    use rusqlite::{Connection, OptionalExtension};

    use crate::types::{self, homework::AdditionalMaterial};
    pub struct Database {
        connection: Connection,
    }
    use rusqlite::Result;

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
            Ok(Self { connection })
        }
        pub fn additional_homework_material(
            &self,
            id: &str,
        ) -> Result<Option<types::homework::AdditionalMaterial>> {
            let mut stmt = self
                .connection
                .prepare("SELECT * FROM additional_homework_materials WHERE id = ?1")?;

            stmt.query_one([id], |x| {
                Ok(AdditionalMaterial {
                    id: x.get_unwrap(0),
                    title: x.get_unwrap(1),
                    urls: {
                        let string: String = x.get_unwrap(2);
                        serde_json::from_str(&string).unwrap()
                    },
                })
            })
            .optional()
        }
        pub fn insert_additional_homework_material(
            &self,
            material: AdditionalMaterial,
        ) -> Result<()> {
            self.connection.execute(
                "INSERT OR REPLACE INTO additional_homework_materials (id, title, urls)
                    VALUES (?1, ?2, ?3)",
                (
                    material.id,
                    material.title,
                    serde_json::to_string_pretty(&material.urls).unwrap(),
                ),
            )?;
            Ok(())
        }
    }
}

fn main() -> anyhow::Result<()> {
    use database::*;
    let db = Database::open()?;
    db.insert_additional_homework_material(types::homework::AdditionalMaterial {
        id: "asdf".into(),
        title: Some("hi".into()),
        urls: Vec::new(),
    })?;
    db.insert_additional_homework_material(types::homework::AdditionalMaterial {
        id: "qweroi".into(),
        title: None,
        urls: vec!["https://google.com/".into()],
    })?;

    dbg!(db.additional_homework_material("asdf")?);

    // dotenvy::dotenv()?;
    // let client = ApiClient::new(std::env::var("MOSRU_BEARER")?);
    // let student_id = 31823383;

    // let endpoint = api::Schedule {
    //     student_id,
    //     dates: _all_possible_dates(),
    // };
    // let response = client.trigger_endpoint(endpoint)?;
    // // dbg!(response);
    // // std::fs::write("homeworks.json", serde_json::to_string_pretty(&response)?)?;

    Ok(())
}

fn _register(storage: &mut BTreeMap<String, u32>, key: impl ToString) {
    *storage.entry(key.to_string()).or_default() += 1;
}

fn _register_maybe(storage: &mut BTreeMap<String, u32>, key: Option<impl ToString>) {
    _register(storage, key.map(|x| x.to_string()).unwrap_or("none".into()));
}

fn _all_possible_dates() -> Vec<Date> {
    let mut dates = Vec::new();
    let start = Date {
        year: 2024,
        month: 9,
        day: 1,
    };
    let end = Date {
        year: 2025,
        month: 5,
        day: 31,
    };
    Date::iterate_days_inclusive(start, end, |date| dates.push(date));
    dates
}
