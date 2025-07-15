use serde::Deserialize;

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
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LessonEducationType {
    Oo,
    Ec,
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
