use backend::db::{
    complete_task as internal_complete_task, create_task, delete_task as internal_delete_task,
    establish_connection, query_tasks,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "complete" => complete_task(&args[2..]),
        "delete" => delete_task(&args[2..]),
        _ => help(),
    }
}

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new task");
    println!("          show: show all tasks");
    println!("  complete<id>: mark a task as done");
    println!("    delete<id>: delete a task from the list");
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
    println!("Task created!");
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("TASKS\n-----");
    for task in query_tasks(&conn) {
        let status = match task.done {
            true => "DONE",
            false => "TODO",
        };
        println!("{}.) {}: {}", task.id, status, task.title);
    }
}

fn complete_task(args: &[String]) {
    if args.len() < 1 {
        println!("complete: missing <id>");
        help();
        return;
    }

    let conn = establish_connection();
    let id = &args[0].parse::<i32>().expect("Invalid ID");
    internal_complete_task(&conn, *id);
    println!("Task completed!");
}

fn delete_task(args: &[String]) {
    if args.len() < 1 {
        println!("delete: missing <id>");
        help();
        return;
    }

    let conn = establish_connection();
    let id = &args[0].parse::<i32>().expect("Invalid ID");
    internal_delete_task(&conn, *id);
    println!("Task deleted!");
}
