// src/repository.rs
use diesel::prelude::*;
use crate::models::User;
use crate::schema::users;
use crate::db::establish_connection;

pub fn get_users() -> Vec<User> {
    let mut connection = establish_connection();
    
    users::table
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users")
}