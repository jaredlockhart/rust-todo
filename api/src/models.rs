#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub done: bool,
}

use super::schema::todos;

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub body: &'a str,
}