use diesel::prelude::*;

use crate::app::models::form::Form;
use crate::app::models::paper_answer::{NewPaperAnswer, PaperAnswer};
use crate::app::models::answer::Answer;
use crate::app::models::user::User;
use crate::config::database::Db;

use crate::database::schema::paper_answers;
use crate::database::schema::answers;
use crate::database::schema::usuarios;
use crate::database::schema::papers;
use crate::database::schema::forms;

// may be this methos sould resolve user and form
pub async fn get_answers(db: &Db, paper_id: i32) -> Vec<Answer> {
    let paper_answers: Vec<PaperAnswer> = db
        .run(move |conn| {
            paper_answers::table
                .filter(paper_answers::paper_id.eq(paper_id))
                .load::<PaperAnswer>(conn)
        })
        .await
        .unwrap();

    let answers: Vec<Answer> = db
        // There is other way to proceed in form_question_repo
        .run(move |conn| {
            answers::table
                .filter(
                    answers::id.eq_any(
                        paper_answers
                            .iter()
                            .map(|pa| pa.answer_id))
                  )
                .load::<Answer>(conn)
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

pub async fn update_answers(db: Db, paper_id: i32, new_answers: Vec<Answer>) -> Vec<Answer> {
    remove_old_answers(&db, paper_id).await;

    let answers = save_answers(db, paper_id, new_answers).await;

    answers
}

// funciones privadas

async fn save_answers(db: Db, paper_id: i32, new_answers: Vec<Answer>) -> Vec<Answer> {
    let paper_answers: Vec<NewPaperAnswer> = new_answers
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
    })
    .await
    .unwrap();

    get_answers(&db, paper_id).await
}

async fn remove_old_answers(db: &Db, paper_id: i32) -> Vec<PaperAnswer> {
    let paper_answers: Vec<PaperAnswer> = db
        .run(move |conn| {
            diesel::delete(paper_answers::table.filter(paper_answers::paper_id.eq(paper_id)))
                .get_results::<PaperAnswer>(conn)
        })
        .await
        .unwrap();

    paper_answers
}
