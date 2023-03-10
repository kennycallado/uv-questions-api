use crate::database::schema::questions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Question {
    pub id: i32,
    pub q_type: String,
    pub question: String,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub q_type: String,
    pub question: String,
}
