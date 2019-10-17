use super::schema::tasks;

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub done: bool,
}
