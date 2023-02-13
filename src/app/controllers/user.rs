use rocket::serde::json::Json;

use crate::app::models::user::User;
use crate::config::database::Db;

use crate::app::repositories::user as user_repo;

#[options("/<id>")]
pub async fn log_post_req(id: i32) {
    println!("AUTH: Update user fmc_token: {}", id);
}

#[get("/")]
pub async fn index(db: Db) -> Json<Vec<User>> {
    let users: Vec<User> = user_repo::find_all(db).await;

    Json(users)
}

#[get("/<id>")]
pub async fn show(db: Db, id: i32) -> Json<User> {
    let user: User = user_repo::find(db, id).await;

    Json(user)
}

#[put("/<id>", data = "<user>")]
pub async fn update(db: Db, id: i32, user: Json<User>) -> Json<User> {
    let user: User = user_repo::update(db, id, user.into_inner()).await;

    Json(user)
}

#[post("/<id>", data ="<token>")]
pub async fn update_token(db: Db, id: i32, token: Json<String>) -> Json<User> {
    let user: User = user_repo::update_token(&db, id, token.into_inner()).await;

    Json(user)
}
