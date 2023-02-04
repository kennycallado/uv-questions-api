use rocket::serde::json::Json;

use crate::app::models::form::{Form, FormWithQuestions, NewForm};
use crate::app::models::question::Question;
use crate::app::repositories::form as form_repo;
use crate::config::database::Db;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<FormWithQuestions>> {
    let forms: Vec<FormWithQuestions> = form_repo::find_all(db).await;

    Json(forms)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<FormWithQuestions> {
    // let form: Form = form_repo::find(db, id).await;
    let form: FormWithQuestions = form_repo::find(db, id).await;

    Json(form)
}

#[get("/<id>/questions")]
pub async fn show_with_questions(db: Db, id: i32) -> Json<(Form, Vec<Question>)> {
    let (form, questions): (Form, Vec<Question>) = form_repo::find_with_questions(db, id).await;

    Json((form, questions))
}

#[post("/", data = "<form>")]
pub async fn store(db: Db, form: Json<NewForm>) -> Json<Form> {
    let form: Form = form_repo::save(db, form.into_inner()).await;

    Json(form)
}

#[delete("/<id>")]
pub async fn destroy(db: Db, id: i32) -> Json<Form> {
    let form: Form = form_repo::remove(db, id).await;

    Json(form)
}

#[put("/<id>", data = "<form>")]
pub async fn update(db: Db, id: i32, form: Json<NewForm>) -> Json<Form> {
    let form: Form = form_repo::update(db, id, form.into_inner()).await;

    Json(form)
}
