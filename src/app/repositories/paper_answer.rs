use diesel::prelude::*;

use crate::app::models::answer::{Answer, AnswerWithQuestion, NewAnswer};
use crate::app::models::form::Form;
use crate::app::models::paper_answer::{NewPaperAnswer, PaperAnswer};
use crate::app::models::question::Question;
use crate::app::models::user::User;
use crate::config::database::Db;

use crate::database::schema::answers;
use crate::database::schema::forms;
use crate::database::schema::paper_answers;
use crate::database::schema::papers;
use crate::database::schema::questions;
use crate::database::schema::usuarios;

use crate::app::repositories::answer as answer_repo;

// may be this methos sould resolve user and form
pub async fn get_answers(db: &Db, paper_id: i32) -> Vec<AnswerWithQuestion> {
    let answers: Vec<AnswerWithQuestion> = db
        .run(move |conn| {
            let answers = paper_answers::table
                .filter(paper_answers::paper_id.eq(paper_id))
                .load::<PaperAnswer>(conn);

            answers.map(|answers| {
                answers
                    .into_iter()
                    .map(|answer| {
                        let answer = answers::table
                            .find(answer.answer_id)
                            .first::<Answer>(conn)
                            .unwrap();

                        let question = questions::table
                            .find(answer.question_id)
                            .first::<Question>(conn)
                            .unwrap();

                        AnswerWithQuestion {
                            id: answer.id,
                            answer: answer.answer,
                            question,
                        }
                    })
                    .collect()
            })
        })
        .await
        .unwrap();

    answers
}

pub async fn get_user(db: &Db, paper_id: i32) -> User {
    let user: User = db
        .run(move |conn| {
            papers::table
                .find(paper_id)
                .inner_join(usuarios::table)
                .select(usuarios::all_columns)
                .first::<User>(conn)
        })
        .await
        .unwrap();

    user
}

pub async fn get_form(db: &Db, paper_id: i32) -> Form {
    let form: Form = db
        .run(move |conn| {
            papers::table
                .find(paper_id)
                .inner_join(forms::table)
                .select(forms::all_columns)
                .first::<Form>(conn)
        })
        .await
        .unwrap();

    form
}

pub async fn update_answers(
    db: Db,
    paper_id: i32,
    new_answers: Vec<NewAnswer>,
) -> Vec<AnswerWithQuestion> {
    remove_old_answers(&db, paper_id).await;

    let answers = save_answers(db, paper_id, new_answers).await;

    answers
}

pub async fn remove_answers(db: &Db, paper_id: i32) -> Vec<AnswerWithQuestion> {
    remove_old_answers(&db, paper_id).await;

    let answers = get_answers(&db, paper_id).await;

    answers
}

// funciones privadas

async fn save_answers(
    db: Db,
    paper_id: i32,
    new_answers: Vec<NewAnswer>,
) -> Vec<AnswerWithQuestion> {
    let answers: Vec<Answer> = answer_repo::save_answers(&db, new_answers).await;

    let paper_answers: Vec<NewPaperAnswer> = answers
        .into_iter()
        .map(|answer| NewPaperAnswer {
            paper_id,
            answer_id: answer.id,
        })
        .collect();

    db.run(move |conn| {
        diesel::insert_into(paper_answers::table)
            .values(paper_answers)
            .execute(conn)
            .unwrap();
    })
    .await;

    get_answers(&db, paper_id).await
}

async fn remove_old_answers(db: &Db, paper_id: i32) -> Vec<Answer> {
    let paper_answers: Vec<PaperAnswer> = db
        .run(move |conn| {
            diesel::delete(paper_answers::table.filter(paper_answers::paper_id.eq(paper_id)))
                .get_results::<PaperAnswer>(conn)
        })
        .await
        .unwrap();

    let ids: Vec<i32> = paper_answers
        .iter()
        .map(|paper_answer| paper_answer.answer_id)
        .collect();

    answer_repo::remove_answers(&db, ids).await
}
