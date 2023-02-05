use rocket::serde::json::Json;

use crate::app::models::answer::{Answer, NewAnswer};
use crate::config::database::Db;

use crate::app::repositories::answer as answer_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<Answer>> {
    let answers: Vec<Answer> = answer_repo::find_all(db).await;

    Json(answers)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Answer> {
    let answer: Answer = answer_repo::find(db, id).await;

    Json(answer)
}

#[post("/", data = "<answer>")]
pub async fn store(db: Db, answer: Json<NewAnswer>) -> Json<Answer> {
    let answer: Answer = answer_repo::save(db, answer.into_inner()).await;

    Json(answer)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Answer> {
    let answer: Answer = answer_repo::remove(db, id).await;

    Json(answer)
}

#[put("/<id>", data = "<answer>")]
pub async fn update(db: Db, id: i32, answer: Json<NewAnswer>) -> Json<Answer> {
    let answer: Answer = answer_repo::update(db, id, answer.into_inner()).await;

    Json(answer)
}
