use diesel::prelude::*;

use crate::app::models::answer::{Answer, NewAnswer};
use crate::config::database::Db;

use crate::database::schema::answers;

pub async fn find_all(db: Db) -> Vec<Answer> {
    let answers: Vec<Answer> = db
        .run(move |conn| answers::table.load::<Answer>(conn))
        .await
        .unwrap();

    answers
}

pub async fn find(db: Db, id: i32) -> Answer {
    let answer: Answer = db
        .run(move |conn| answers::table.find(id).first::<Answer>(conn))
        .await
        .unwrap();

    answer
}

pub async fn save(db: Db, new: NewAnswer) -> Answer {
    let answer: Answer = db
        .run(move |conn| {
            diesel::insert_into(answers::table)
                .values(&new)
                .get_result::<Answer>(conn)
        })
        .await
        .unwrap();

    answer
}

pub async fn remove(db: Db, id: i32) -> Answer {
    let answer: Answer = db
        .run(move |conn| diesel::delete(answers::table.find(id)).get_result::<Answer>(conn))
        .await
        .unwrap();

    answer
}

pub async fn update(db: Db, id: i32, new: NewAnswer) -> Answer {
    let answer: Answer = db
        .run(move |conn| {
            let answer = diesel::update(answers::table.find(id));

            answer
                .set((
                    // answers::columns::id.eq(id),
                    answers::columns::question_id.eq(new.question_id),
                    answers::columns::answer.eq(new.answer),
                ))
                .get_result::<Answer>(conn)
        })
        .await
        .unwrap();

    answer
}
