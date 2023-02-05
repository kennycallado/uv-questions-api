use diesel::prelude::*;

use crate::app::models::paper::{Paper, NewPaper};
use crate::config::database::Db;

use crate::database::schema::papers;

pub async fn find_all(db: Db) -> Vec<Paper> {
    let papers: Vec<Paper> = db
        .run(move |conn| papers::table.load::<Paper>(conn))
        .await
        .unwrap();

    papers
}

pub async fn find(db: Db, id: i32) -> Paper {
    let paper: Paper = db
        .run(move |conn| papers::table.find(id).first::<Paper>(conn))
        .await
        .unwrap();

    paper
}

pub async fn save(db: Db, new: NewPaper) -> Paper {
    let paper: Paper = db
        .run(move |conn| { 
            diesel::insert_into(papers::table)
                .values(new)
                .get_result(conn)
        })
        .await
        .unwrap();

    paper
}

pub async fn remove(db: Db, id: i32) -> Paper {
    let paper: Paper = db
        .run(move |conn| {
            diesel::delete(papers::table.find(id))
                .get_result(conn)
        })
        .await
        .unwrap();

    paper
}

pub async fn update(db: Db, id: i32, new: NewPaper) -> Paper {
    let paper: Paper = db
        .run(move |conn| {
            diesel::update(papers::table.find(id))
                .set((
                    // papers::id.eq(id),
                    papers::user_id.eq(new.user_id),
                    papers::form_id.eq(new.form_id),
                ))
                .get_result::<Paper>(conn)
        })
        .await
        .unwrap();

    paper
}
