use crate::app::models::form_question::{FormQuestion, NewFormQuestion};
use crate::config::database::Db;

use crate::database::schema::form_questions;

use diesel::prelude::*;

pub async fn find_all(db: Db) -> Vec<FormQuestion> {
    let form_questions: Vec<FormQuestion> = db
        .run(move |conn| form_questions::table.load::<FormQuestion>(conn))
        .await
        .unwrap();

    form_questions
}

pub async fn find(db: Db, id: i32) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| form_questions::table.find(id).first::<FormQuestion>(conn))
        .await
        .unwrap();

    form_question
}

pub async fn save(db: Db, new: NewFormQuestion) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| {
            diesel::insert_into(form_questions::table)
                .values(new)
                .get_result::<FormQuestion>(conn)
        })
        .await
        .unwrap();

    form_question
}

pub async fn remove(db: Db, id: i32) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| {
            diesel::delete(form_questions::table.find(id)).get_result::<FormQuestion>(conn)
        })
        .await
        .unwrap();

    form_question
}

pub async fn update(db: Db, id: i32, new: NewFormQuestion) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| {
            let form_question = diesel::update(form_questions::table.find(id));

            form_question
                .set((
                    // form_questions::columns::id.eq(id),
                    form_questions::columns::form_id.eq(new.form_id),
                    form_questions::columns::question_id.eq(new.question_id),
                ))
                .get_result::<FormQuestion>(conn)
        })
        .await
        .unwrap();

    form_question
}
