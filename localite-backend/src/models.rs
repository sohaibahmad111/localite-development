// src/models.rs
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String, 
    pub created_at: NaiveDateTime,
}