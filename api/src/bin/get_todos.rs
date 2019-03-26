extern crate rust_todo;
extern crate diesel;

use self::rust_todo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_todo::schema::todos::dsl::*;

    let connection = establish_connection();
    let results = todos.filter(done.eq(false))
        .limit(5)
        .load::<Todo>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}