use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_task<'a>(connection: &SqliteConnection, title: &'a str) {
    let task = models::NewTask { title };

    diesel::insert_into(schema::tasks::table)
        .values(&task)
        .execute(connection)
        .expect("Error inserting new task");
}

pub fn query_tasks(connection: &SqliteConnection) -> Vec<models::Task> {
    schema::tasks::table
        .load::<models::Task>(connection)
        .expect("Error loading tasks")
}

pub fn complete_task(connection: &SqliteConnection, id: i32) {
    use schema::tasks::dsl::{done, tasks};
    let task = tasks.find(id);

    diesel::update(task)
        .set(done.eq(true))
        .execute(connection)
        .expect("Error completing task");
}

pub fn delete_task(connection: &SqliteConnection, id: i32) {
    use schema::tasks::dsl::tasks;
    let task = tasks.find(id);

    diesel::delete(task)
        .execute(connection)
        .expect("Error deleting task");
}
