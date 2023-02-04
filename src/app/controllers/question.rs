use rocket::serde::json::Json;

use crate::app::models::question::{NewQuestion, Question};
use crate::app::repositories::question as question_repo;
use crate::config::database::Db;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<Question>> {
    let questions: Vec<Question> = question_repo::find_all(db).await;

    Json(questions)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Question> {
    let question: Question = question_repo::find(db, id).await;

    Json(question)
}

#[post("/", data = "<question>")]
pub async fn store(db: Db, question: Json<NewQuestion>) -> Json<Question> {
    let question: Question = question_repo::save(db, question.into_inner()).await;

    Json(question)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Question> {
    let question: Question = question_repo::remove(db, id).await;

    Json(question)
}

#[put("/<id>", data = "<question>")]
pub async fn update(db: Db, id: i32, question: Json<NewQuestion>) -> Json<Question> {
    let question: Question = question_repo::update(db, id, question.into_inner()).await;

    Json(question)
}
