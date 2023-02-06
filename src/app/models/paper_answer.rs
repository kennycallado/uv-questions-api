use crate::database::schema::paper_answers;
use serde::{Deserialize, Serialize};

use crate::app::models::answer::Answer;
use crate::app::models::paper::Paper;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[serde(crate = "rocket::serde")]
#[belongs_to(Paper)]
#[belongs_to(Answer)]
pub struct PaperAnswer {
    pub id: i32,
    pub paper_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "paper_answers"]
pub struct NewPaperAnswer {
    pub paper_id: i32,
    pub answer_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "paper_answers"]
pub struct PaperAnswerWithQuestion {
    pub id: i32,
    pub answer: Answer,
}
