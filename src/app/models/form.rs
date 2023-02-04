use crate::database::schema::forms;
use serde::{Deserialize, Serialize};

use super::question::Question;

#[derive(Debug, Serialize, Clone, Deserialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Form {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "forms"]
pub struct NewForm {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "forms"]
pub struct FormWithQuestions {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}
