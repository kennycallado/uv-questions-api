use rocket::serde::json::Json;

use crate::app::models::paper::{NewPaper, NewPaperWithData, PaperWithAnswers};
use crate::config::database::Db;

use crate::app::repositories::paper as paper_repo;

#[options("/")]
pub async fn log_post_req() {
    println!("AUTH: Create a new paper");
}

#[options("/<id>")]
pub async fn log_put_req(id: i32) {
    println!("AUTH: Update paper id: {}", id);
}

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

#[get("/user/<user_id>")]
pub async fn show_by_user(db: Db, user_id: i32) -> Json<Vec<PaperWithAnswers>> {
    let papers: Vec<PaperWithAnswers> = paper_repo::find_by_user(db, user_id).await;

    Json(papers)
}

#[post("/", data = "<paper>")]
pub async fn store(db: Db, paper: Json<NewPaper>) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::save(db, paper.into_inner()).await;

    Json(paper)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::remove(db, id).await;

    Json(paper)
}

#[put("/<id>", data = "<paper>")]
pub async fn update(db: Db, id: i32, paper: Json<NewPaperWithData>) -> Json<PaperWithAnswers> {
    let paper: PaperWithAnswers = paper_repo::update(db, id, paper.into_inner()).await;

    Json(paper)
}
