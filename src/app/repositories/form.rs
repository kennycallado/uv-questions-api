use diesel::prelude::*;

use crate::app::models::form::{Form, FormWithQuestions, NewForm};
use crate::app::models::form_question::FormQuestion;
use crate::app::models::question::Question;
use crate::config::database::Db;

// Esto no me gusta nada.. en form solo squema de form
// quizá from_question pero ¿questions?. Como mucho en
// el repo de form_questions
use crate::database::schema::{form_questions, forms, questions};

pub async fn find_all(db: Db) -> Vec<FormWithQuestions> {
    let forms: Vec<FormWithQuestions> = db
        .run(move |conn| {
            let forms = forms::table.load::<Form>(conn);

            // let forms_with_questions = forms.map(|forms| {
            forms.map(|forms| {
                forms
                    .into_iter()
                    .map(|form| {
                        let form_questions = form_questions::table
                            .filter(form_questions::form_id.eq(form.id))
                            .load::<FormQuestion>(conn)
                            .unwrap();

                        let questions = form_questions
                            .into_iter()
                            .map(|form_question| {
                                questions::table
                                    .filter(questions::id.eq(form_question.question_id))
                                    .first::<Question>(conn)
                                    .unwrap()
                            })
                            .collect();

                        FormWithQuestions {
                            id: form.id,
                            title: form.title,
                            description: form.description,
                            questions,
                        }
                    })
                    .collect()
            })
        })
        .await
        .unwrap();

    forms
}

pub async fn find(db: Db, id: i32) -> FormWithQuestions {
    let form: Form = db
        .run(move |conn| forms::table.find(id).first::<Form>(conn))
        .await
        .unwrap();

    let questions: Vec<Question> = db
        .run(move |conn| {
            form_questions::table
                .inner_join(questions::table)
                .filter(form_questions::form_id.eq(id))
                .select(questions::all_columns)
                .load::<Question>(conn)
            // questions::table
            //     .inner_join(form_questions::table)
            //     .filter(form_questions::form_id.eq(id))
            //     .select(questions::all_columns)
            //     .load::<Question>(conn)
        })
        .await
        .unwrap();

    FormWithQuestions {
        id: form.id,
        title: form.title,
        description: form.description,
        questions,
    }
}

pub async fn save(db: Db, new: NewForm) -> Form {
    let form: Form = db
        .run(move |conn| {
            diesel::insert_into(forms::table)
                .values(new)
                .get_result::<Form>(conn)
        })
        .await
        .unwrap();

    form
}

pub async fn remove(db: Db, id: i32) -> Form {
    let form: Form = db
        .run(move |conn| diesel::delete(forms::table.find(id)).get_result::<Form>(conn))
        .await
        .unwrap();

    form
}

pub async fn update(db: &Db, id: i32, new: NewForm) -> Form {
    let form: Form = db
        .run(move |conn| {
            let form = diesel::update(forms::table.find(id));

            form.set((
                // forms::columns::id.eq(id),
                forms::columns::title.eq(new.title),
                forms::columns::description.eq(new.description),
            ))
            .get_result::<Form>(conn)
        })
        .await
        .unwrap();

    form
}
