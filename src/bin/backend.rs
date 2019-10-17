#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;
use rust_todo::db::{establish_connection, query_tasks, models::Task};

#[derive(Serialize)]
struct JsonApiReponse {
    data: Vec<Task>,
}

#[get("/tasks")]
fn tasks_get() -> Json<JsonApiReponse> {
    let mut response = JsonApiReponse { data: vec![] };

    let conn = establish_connection();
    for task in query_tasks(&conn) {
        response.data.push(task);
    }

    Json(response)
}

fn main() {
    rocket::ignite().mount("/", routes![tasks_get]).launch();
}
