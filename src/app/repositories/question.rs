use crate::config::database::Db;
use crate::app::models::question::Question;

use crate::database::schema::questions;
// use crate::database::schema::questions::dsl::*;

use diesel::prelude::*;

pub async fn find_all(db: Db) -> Vec<Question> {
    let questions: Vec<Question> = db
        .run(move |conn| questions::table.load::<Question>(conn) )
        .await
        .unwrap();

    questions
}

pub async fn find(db: Db, id: i32) -> Question {
  let question= db
    .run(move |conn| questions::table.find(id).get_result::<Question>(conn)).await.unwrap();

  question
}

