use diesel::prelude::*;

use crate::app::models::answer::Answer;
use crate::app::models::form::Form;
use crate::app::models::paper::{NewPaper, Paper, PaperWithAnswers, PaperWithAnswersData};
use crate::app::models::user::User;
use crate::config::database::Db;

use crate::database::schema::papers;

use crate::app::repositories::paper_answer as paper_answer_repo;

pub async fn find_all(db: Db) -> Vec<PaperWithAnswers> {
    let papers: Vec<Paper> = db
        .run(move |conn| papers::table.load::<Paper>(conn))
        .await
        .unwrap();

    let mut papers_with_answers: Vec<PaperWithAnswers> = Vec::new();
    for paper in papers {
        let answers: Vec<Answer> = paper_answer_repo::get_answers(&db, paper.id).await;
        let user: User = paper_answer_repo::get_user(&db, paper.id).await;
        let form: Form = paper_answer_repo::get_form(&db, paper.id).await;

        papers_with_answers.push(PaperWithAnswers {
            id: paper.id,
            form,
            user,
            answers,
        });
    }

    papers_with_answers
}

pub async fn find(db: Db, id: i32) -> PaperWithAnswers {
    let paper: Paper = db
        .run(move |conn| papers::table.find(id).first::<Paper>(conn))
        .await
        .unwrap();

    let answers: Vec<Answer> = paper_answer_repo::get_answers(&db, paper.id).await;
    let user: User = paper_answer_repo::get_user(&db, paper.id).await;
    let form: Form = paper_answer_repo::get_form(&db, paper.id).await;

    PaperWithAnswers {
        id: paper.id,
        user,
        form,
        answers,
    }
}

pub async fn save(db: Db, new: NewPaper) -> PaperWithAnswers {
    let paper: Paper = db
        .run(move |conn| {
            diesel::insert_into(papers::table)
                .values(new)
                .get_result::<Paper>(conn)
        })
        .await
        .unwrap();

    let user: User = paper_answer_repo::get_user(&db, paper.id).await;
    let form: Form = paper_answer_repo::get_form(&db, paper.id).await;
    let answers: Vec<Answer> = paper_answer_repo::get_answers(&db, paper.id).await;

    PaperWithAnswers {
        id: paper.id,
        user,
        form,
        answers,
  }
}

pub async fn remove(db: Db, id: i32) -> Paper {
    let paper: Paper = db
        .run(move |conn| diesel::delete(papers::table.find(id)).get_result(conn))
        .await
        .unwrap();

    paper
}

pub async fn update(db: Db, id: i32, new: PaperWithAnswersData) -> PaperWithAnswers {
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

        let user: User = paper_answer_repo::get_user(&db, paper.id).await;
        let form: Form = paper_answer_repo::get_form(&db, paper.id).await;
        let answers = paper_answer_repo::update_answers(db, id, new.answers.clone()).await;

    PaperWithAnswers {
        id: paper.id,
        user,
        form,
        answers,
    }
}
