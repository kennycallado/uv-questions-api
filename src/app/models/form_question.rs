use crate::database::schema::form_questions;
use serde::{Deserialize, Serialize};

use crate::app::models::form::Form;
use crate::app::models::question::Question;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Form)]
#[belongs_to(Question)]
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
