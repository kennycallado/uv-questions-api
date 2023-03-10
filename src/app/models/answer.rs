use crate::database::schema::answers;
use serde::{Deserialize, Serialize};

use crate::app::models::question::Question;

#[derive(Debug, Serialize, Clone, Deserialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub answer: String,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "answers"]
pub struct NewAnswer {
    pub question_id: i32,
    pub answer: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "answers"]
pub struct AnswerWithQuestion {
    pub id: i32,
    pub question: Question,
    pub answer: String,
}
