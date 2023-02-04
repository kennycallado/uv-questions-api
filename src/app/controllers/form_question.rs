use rocket::serde::json::Json;

use crate::app::models::form_question::{FormQuestion, NewFormQuestion};
use crate::app::repositories::form_question as form_question_repo;
use crate::config::database::Db;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<FormQuestion>> {
    let form_questions = form_question_repo::find_all(db).await;

    Json(form_questions)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<FormQuestion> {
    let form_question = form_question_repo::find(db, id).await;

    Json(form_question)
}

#[post("/", data = "<form_question>")]
pub async fn store(db: Db, form_question: Json<NewFormQuestion>) -> Json<FormQuestion> {
    let form_question = form_question_repo::save(db, form_question.into_inner()).await;

    Json(form_question)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<FormQuestion> {
    let form_question = form_question_repo::remove(db, id).await;

    Json(form_question)
}

#[put("/<id>", data = "<form_question>")]
pub async fn update(db: Db, id: i32, form_question: Json<NewFormQuestion>) -> Json<FormQuestion> {
    let form_question = form_question_repo::update(db, id, form_question.into_inner()).await;

    Json(form_question)
}
