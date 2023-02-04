use rocket::serde::json::Json;

use crate::app::models::form::{Form, FormWithQuestions, NewForm};
use crate::config::database::Db;

use crate::app::repositories::form as form_repo;
use crate::app::repositories::form_question as form_question_repo;

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<FormWithQuestions>> {
    let forms: Vec<FormWithQuestions> = form_repo::find_all(db).await;

    Json(forms)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<FormWithQuestions> {
    let form: FormWithQuestions = form_repo::find(db, id).await;

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

// #[put("/<id>", data = "<form>")]
// pub async fn update(db: Db, id: i32, form: Json<NewForm>) -> Json<Form> {
//     let form: Form = form_repo::update(db, id, form.into_inner()).await;

//     Json(form)
// }

#[put("/<id>", data = "<new_form>")]
pub async fn update(db: Db, id: i32, new_form: Json<FormWithQuestions>) -> Json<FormWithQuestions> {
    // update the form itself
    let form: NewForm = NewForm {
        title: new_form.title.clone(),
        description: new_form.description.clone(),
    };
    form_repo::update(&db, id, form).await;

    // update the questions
    let questions = form_question_repo::update_questions(db, id, new_form.questions.clone()).await;

    // Prepare the response
    let form = FormWithQuestions {
        id: new_form.id,               // Ojo, esto no es real
        title: new_form.title.clone(), // Esto sí es real
        description: new_form.description.clone(),
        questions,
    };

    Json(form)
}
