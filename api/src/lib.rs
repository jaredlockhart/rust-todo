#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Todo, NewTodo};

pub fn create_todo<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Todo {
    use schema::todos;

    let new_post = NewTodo {
        title: title,
        body: body,
    };

    diesel::insert_into(todos::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}