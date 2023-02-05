use diesel::prelude::*;

use crate::app::models::user::User;
use crate::config::database::Db;

use crate::database::schema::usuarios;

pub async fn find_all(db: Db) -> Vec<User> {
    let users = db
        .run(move |conn| usuarios::table.load::<User>(conn))
        .await
        .unwrap();

    users
}

pub async fn find(db: Db, id: i32) -> User {
    let user = db
        .run(move |conn| usuarios::table.find(id).get_result::<User>(conn))
        .await
        .unwrap();

    user
}

pub async fn update(db: Db, id: i32, new: User) -> User {
    let user: User = db
        .run(move |conn| {
            let user = diesel::update(usuarios::table.find(id));

            user.set((
                usuarios::columns::email.eq(new.email),
                usuarios::columns::token.eq(new.token),
            ))
            .get_result::<User>(conn)
        })
        .await
        .unwrap();

    user
}
