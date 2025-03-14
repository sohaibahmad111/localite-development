// src/repository.rs
use diesel::prelude::*;
use diesel::SqliteConnection;
use crate::models::User;
use crate::schema::users;

pub fn get_users() -> Vec<User> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let mut connection = SqliteConnection::establish(&database_url)
        .expect("Failed to connect to database");
    
    users::table
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users")
}