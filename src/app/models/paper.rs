use crate::database::schema::papers;
use serde::{Deserialize, Serialize};

use crate::app::models::form::Form;
use crate::app::models::user::User;
use crate::app::models::answer::Answer;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(User)]
#[belongs_to(Form)]
pub struct Paper {
    pub id: i32,
    pub user_id: i32,
    pub form_id: i32,
}

#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "papers"]
pub struct NewPaper {
    pub user_id: i32,
    pub form_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "papers"]
pub struct PaperWithAnswers {
    pub id: i32,
    pub user: User,
    pub form: Form,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "papers"]
pub struct PaperWithAnswersData {
    pub id: i32,
    pub user_id: i32,
    pub form_id: i32,
    pub answers: Vec<Answer>,
}
