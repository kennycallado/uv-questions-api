use rocket::serde::json::Json;

use crate::config::database::Db;
use crate::app::models::form::{Form, NewForm};
use crate::app::repositories::form as form_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<Form>> {
  let forms: Vec<Form> = form_repo::find_all(db).await;

  Json(forms)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<Form> {
  let form: Form = form_repo::find(db, id).await;

  Json(form)
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
