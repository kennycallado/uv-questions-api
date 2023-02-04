use crate::app::models::form::{Form, NewForm};
use crate::config::database::Db;

use crate::database::schema::forms;

use diesel::prelude::*;

pub async fn find_all(db: Db) -> Vec<Form> {
    let forms: Vec<Form> = db
        .run(move |conn| forms::table.load::<Form>(conn))
        .await
        .unwrap();

    forms
}

pub async fn find(db: Db, id: i32) -> Form {
    let form: Form = db
        .run(move |conn| forms::table.find(id).first::<Form>(conn))
        .await
        .unwrap();

    form
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

pub async fn update(db: Db, id: i32, new: NewForm) -> Form {
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
