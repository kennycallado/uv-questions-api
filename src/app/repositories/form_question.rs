use diesel::prelude::*;

use crate::app::models::form_question::{FormQuestion, NewFormQuestion};
use crate::app::models::question::Question;
use crate::config::database::Db;

use crate::database::schema::form_questions;
use crate::database::schema::questions;

pub async fn get_questions(db: Db, form_id: i32) -> Vec<Question> {
    let form_questions: Vec<FormQuestion> = db
        .run(move |conn| {
            form_questions::table
                .filter(form_questions::form_id.eq(form_id))
                .load::<FormQuestion>(conn)
        })
        .await
        .unwrap();

    let questions: Vec<Question> = db
        .run(move |conn| {
            form_questions
                .into_iter()
                .map(|form_question| {
                    questions::table
                        .filter(questions::id.eq(form_question.question_id))
                        .first::<Question>(conn)
                        .unwrap()
                })
                .collect::<Vec<Question>>()
        })
        .await;

    questions
}

pub async fn update_questions(db: Db, form_id: i32, new_questions: Vec<Question>) -> Vec<Question> {
    remove_old_questions(&db, form_id).await;

    let questions = save_questions(db, form_id, new_questions).await;

    questions
}

// funciones privadas

async fn save_questions(db: Db, form_id: i32, new_questions: Vec<Question>) -> Vec<Question> {
    let form_questions: Vec<NewFormQuestion> = new_questions
        .into_iter()
        .map(|question| NewFormQuestion {
            form_id,
            question_id: question.id,
        })
        .collect();

    db.run(move |conn| {
        diesel::insert_into(form_questions::table)
            .values(form_questions)
            .execute(conn)
            .unwrap();
    })
    .await;

    let questions = get_questions(db, form_id).await;

    questions
}

async fn remove_old_questions(db: &Db, form_id: i32) -> Vec<FormQuestion> {
    let form_questions: Vec<FormQuestion> = db
        .run(move |conn| {
            diesel::delete(form_questions::table.filter(form_questions::form_id.eq(form_id)))
                .get_results::<FormQuestion>(conn)
        })
        .await
        .expect("Error deleting form questions");

    form_questions
}

// Nada de abajo se usa

async fn save(db: &Db, new: NewFormQuestion) -> FormQuestion {
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

async fn find_all(db: Db) -> Vec<FormQuestion> {
    let form_questions: Vec<FormQuestion> = db
        .run(move |conn| form_questions::table.load::<FormQuestion>(conn))
        .await
        .unwrap();

    form_questions
}

async fn find(db: Db, id: i32) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| form_questions::table.find(id).first::<FormQuestion>(conn))
        .await
        .unwrap();

    form_question
}

async fn remove(db: Db, id: i32) -> FormQuestion {
    let form_question: FormQuestion = db
        .run(move |conn| {
            diesel::delete(form_questions::table.find(id)).get_result::<FormQuestion>(conn)
        })
        .await
        .unwrap();

    form_question
}

async fn update(db: Db, id: i32, new: NewFormQuestion) -> FormQuestion {
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
