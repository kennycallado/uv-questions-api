use rocket::serde::json::Json;

use crate::app::models::paper::{NewPaper, Paper, PaperWithAnswers, PaperWithAnswersData};
use crate::config::database::Db;

use crate::app::repositories::paper as paper_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<PaperWithAnswers>> {
    let papers: Vec<PaperWithAnswers> = paper_repo::find_all(db).await;

    Json(papers)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::find(db, id).await;

    Json(paper)
}

#[post("/", data = "<paper>")]
pub async fn store(db: Db, paper: Json<NewPaper>) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::save(db, paper.into_inner()).await;

    Json(paper)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Paper> {
    let paper: Paper = paper_repo::remove(db, id).await;

    Json(paper)
}

#[put("/<id>", data = "<paper>")]
pub async fn update(db: Db, id: i32, paper: Json<PaperWithAnswersData>) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::update(db, id, paper.into_inner()).await;

    Json(paper)
}
