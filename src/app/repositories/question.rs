use crate::config::database::Db;
use crate::app::models::question::{Question, NewQuestion};

use crate::database::schema::questions;
// use crate::database::schema::questions::dsl::*;

use diesel::prelude::*;

pub async fn find_all(db: Db) -> Vec<Question> {
    let questions: Vec<Question> = db
        .run(move |conn| questions::table.load::<Question>(conn))
        .await
        .unwrap();

    questions
}

pub async fn find(db: Db, id: i32) -> Question {
  let question= db
    .run(move |conn| questions::table.find(id).get_result::<Question>(conn))
    .await
    .unwrap();

  question
}

pub async fn save(db: Db, new: NewQuestion) -> Question {
  let question: Question = db
    .run(move |conn| diesel::insert_into(questions::table).values(new).get_result::<Question>(conn))
    .await
    .unwrap();

  question
}

pub async fn remove(db: Db, id: i32) -> Question {
  let question: Question = db
    .run(move |conn| diesel::delete(questions::table.find(id)).get_result::<Question>(conn))
    .await
    .unwrap();

  question
}

pub async fn update(db: Db, id: i32, new: NewQuestion) -> Question {
  let question: Question = db
    .run(move |conn| {
      let question = diesel::update(questions::table.find(id));

      question.set((
        // questions::columns::id.eq(id),
        questions::columns::q_type.eq(new.q_type),
        questions::columns::question.eq(new.question),
      ))
      .get_result::<Question>(conn)
    })
    .await
    .unwrap();

  question
}
