use std::{error::Error, fmt::Display};

use rusqlite::{ToSql, types::FromSql};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseLessonType {
    ThematicTest,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeseaseStatusType {
    Sick,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum LessonEducationType {
    #[serde(rename = "OO")]
    Normal,
    #[serde(rename = "EC")]
    ExtraActivity,
}
impl Display for LessonEducationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Normal => "OO",
                Self::ExtraActivity => "EC",
            }
        )
    }
}
impl ToSql for LessonEducationType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Text(
                match self {
                    LessonEducationType::Normal => "normal",
                    LessonEducationType::ExtraActivity => "extra activity",
                }
                .to_owned(),
            ),
        ))
    }
}
impl FromSql for LessonEducationType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(items) => match items {
                b"normal" => Ok(Self::Normal),
                b"extra activity" => Ok(Self::ExtraActivity),
                _ => Err(rusqlite::types::FromSqlError::Other(
                    anyhow::anyhow!("unknown lesson type `{}`", String::from_utf8_lossy(items))
                        .into_boxed_dyn_error(),
                )),
            },
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LessonType {
    Normal,
    Remote,
    #[serde(rename = "")]
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HomeworkType {
    Oo,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalMaterialSelectedMode {
    Execute,
    Learn,
}

#[derive(Debug, Deserialize)]
pub enum AdditionalMaterialName {
    #[serde(rename = "Облако знаний")]
    KnowledgeCloud,
    #[serde(rename = "Приложение")]
    Application,
    #[serde(rename = "Сценарий урока")]
    LessonScenario,
    #[serde(rename = "Тест")]
    Test,
    #[serde(rename = "Тестовая спецификация (Тест)")]
    TestSpecification,
    #[serde(rename = "Цифровой учитель")]
    DigitalTeacher,
    #[serde(rename = "Другое")]
    Other1,
    #[serde(rename = "Прочее")]
    Other2,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalMaterialType {
    AtomicObject,
    Attachments,
    BasicMaterial,
    FizikonModule,
    GameApp,
    LessonTemplate,
    TestSpecBinding,
    TestSpecification,
    TestTaskBinding,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrlType {
    FileLink,
}
