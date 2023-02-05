use crate::database::schema::usuarios;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
#[table_name = "usuarios"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub token: String,
}

/*
#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "usuarios"]
pub struct UserWithAnswers {
    pub id: i32,
    pub email: String,
    pub toekn: String,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct FormsAnswers {
    pub id: i32,
    pub form_id: i32,
    pub question_id: i32,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable)]
pub struct Answer {
    pub question_id: i32,
    pub answer: i32,
}
*/
