use crate::database::schema::forms;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Form {
  pub id: i32,
  pub title: String,
  pub description: String,
}


#[derive(Debug, Clone, FromForm, Deserialize, Serialize, Insertable)]
#[table_name = "forms"]
pub struct NewForm {
  pub title: String,
  pub description: String,
}
