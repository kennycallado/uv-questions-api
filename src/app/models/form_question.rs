use crate::database::schema::form_questions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct FormQuestion {
    pub id: i32,
    pub form_id: i32,
    pub question_id: i32,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "form_questions"]
pub struct NewFormQuestion {
    pub form_id: i32,
    pub question_id: i32,
}
