use diesel::prelude::*;

use crate::app::models::answer::{Answer, NewAnswer, AnswerWithQuestion};
use crate::app::models::question::Question;
use crate::config::database::Db;

use crate::database::schema::answers;
use crate::database::schema::questions;

pub async fn find_all(db: Db) -> Vec<AnswerWithQuestion> {
    let answers: Vec<AnswerWithQuestion> = db
        .run(move |conn| {
            let answers = answers::table.load::<Answer>(conn);
      
            answers.map(|answers| {
                answers
                    .into_iter()
                    .map(|answer| {
                        let question = questions::table
                            .find(answer.question_id)
                            .first::<Question>(conn)
                            .unwrap();

                        AnswerWithQuestion {
                            id: answer.id,
                            question,
                            answer: answer.answer,
                        }
                    })
                    .collect()
            })
        })
        .await
        .unwrap();

    answers
}

pub async fn find(db: Db, id: i32) -> AnswerWithQuestion {
    let answer: AnswerWithQuestion = db
        .run(move |conn| {
            let answer = answers::table.find(id).first::<Answer>(conn);
            answer.map(|answer| {
                let question = questions::table
                    .find(answer.question_id)
                    .first::<Question>(conn)
                    .unwrap();

                AnswerWithQuestion {
                    id: answer.id,
                    question,
                    answer: answer.answer,
                }
            })
        })
        .await
        .unwrap();

    answer
}

pub async fn save(db: Db, new: NewAnswer) -> AnswerWithQuestion {
    let answer: AnswerWithQuestion = db
        .run(move |conn| {
            let answer: Answer = diesel::insert_into(answers::table)
                .values(&new)
                .get_result::<Answer>(conn).unwrap();

            let question: Question = questions::table
                .find(answer.question_id)
                .first::<Question>(conn).unwrap();
      
            AnswerWithQuestion {
                id: answer.id,
                question,
                answer: answer.answer,
            }
        })
        .await;
    answer
}

pub async fn save_answers(db: &Db, new: Vec<NewAnswer>) -> Vec<Answer> {
    let answers: Vec<Answer> = db
        .run(move |conn| {
            diesel::insert_into(answers::table)
                .values(&new)
                .get_results::<Answer>(conn)
        })
        .await
        .unwrap();

    answers
}

pub async fn remove(db: Db, id: i32) -> Answer {
    let answer: Answer = db
        .run(move |conn| diesel::delete(answers::table.find(id)).get_result::<Answer>(conn))
        .await
        .unwrap();

    answer
}

pub async fn update(db: Db, id: i32, new: NewAnswer) -> AnswerWithQuestion {
    let answer: AnswerWithQuestion = db
        .run(move |conn| {
            let answer = diesel::update(answers::table.find(id));

            let answer: Answer = answer
                .set((
                    // answers::columns::id.eq(id),
                    answers::columns::question_id.eq(new.question_id),
                    answers::columns::answer.eq(new.answer),
                ))
                .get_result::<Answer>(conn).unwrap();

            let question = questions::table
                .find(answer.question_id)
                .first::<Question>(conn).unwrap();

            AnswerWithQuestion {
                id: answer.id,
                question,
                answer: answer.answer,
            }
        })
        .await;

    answer
}
