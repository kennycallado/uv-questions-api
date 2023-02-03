use rocket::serde::json::Json;

use crate::config::database::Db;
use crate::app::models::question::Question;
use crate::app::repositories::question as question_repo;

#[get("/all")]
pub async fn index(db: Db) -> Json<Vec<Question>> {
  let questions: Vec<Question> = question_repo::find_all(db).await;

  Json(questions)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Question> {
  let question: Question = question_repo::find(db, id).await;

  Json(question)
}

