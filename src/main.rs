use chrono::{prelude::*, NaiveDate};
use rusqlite::{params, Connection, Result};
use std::{env, fs};
use dirs::home_dir;

#[derive(Debug)]
struct Task {
    id: Option<i32>,
    name: String,
    is_done: Option<bool>,
    date: String,
}

fn main() {
    let conn = match database_connection() {
        Ok(conn) => conn,
        Err(_e) => {
            panic!()
        }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Use help command to see the available commands");
    } else {
        let com: &str = &args[1];
        let mut _res: Result<()>;
        match com {
            "list" => {
                if args.len() == 2 {
                    _res = list_all_tasks(&conn);
                } else {
                    let is_done = match args[2].as_str() {
                        "done" => 1,
                        "not-done" => 0,
                        _ => -1,
                    };
                    if is_done == -1 {
                        println!("Invalid input for tasks")
                    } else {
                        _res = list_tasks(&conn, is_done);
                    }
                }
            }
            "add" => {
                match args.len() {
                    4 => {
                        if validate_date(&args[3].clone()){
                            let task = Task {
                                id: None,
                                name: args[2].clone(),
                                is_done: None,
                                date: args[3].clone(),
                        };

                        _res = add_task(&conn, task);
                        }else{
                            println!("Please insert correct date format (%d-%m-%Y)")
                        }
                    },
                    3 => {
                        let local: DateTime<Local> = Local::now();
                        let date = local.format("%d-%m-%Y").to_string();
                        let task = Task {
                            id: None,
                            name: args[2].clone(),
                            is_done: None,
                            date,
                        };
                        _res = add_task(&conn, task);
                    }
                    _=>{println!("Please provide correct information format for your task!(use do help for help)");}

                    }
            }
            "done" => _res = mark_done(&conn, args[2].parse::<i32>().unwrap()),
            "undone" => _res = mark_undone(&conn, args[2].parse::<i32>().unwrap()),
            "help" => println!(" help -> help command \n list -> list all task \n list done -> to list done tasks \n list not-done -> to list not-done tasks \n add `task-name` `date` -> to add task, date foramt %d-%m-%Y, leave date blank to use current date\n done `task-id` -> mark a task as done\n undone `task-id` -> mark a task as undone\nremove `task-id` -> remove a task"),
            "remove" => _res = remove_task(&conn, args[2].parse::<i32>().unwrap()),
            _ => println!("Wrongg"),
        }
    }
}

fn database_connection() -> Result<Connection> {
    let mut db_path = home_dir().unwrap();
    db_path.push("var/lib/do/do_database.db");
    if let Some(parent) = db_path.parent() {
        let _ =fs::create_dir_all(parent);
    }
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL,
                is_done  INTEGER DEFAULT 0,
                date  TEXT DEFAULT NULL
            )",
        [],
    )?;
    Ok(conn)
}

fn add_task(conn: &Connection, task: Task) -> Result<()> {
    conn.execute(
        "INSERT INTO task (name, date) VALUES (?1, ?2)",
        params![task.name, task.date],
    )?;
    Ok(())
}
fn list_tasks(conn: &Connection, is_done: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, is_done, date FROM task WHERE is_done = ?1")?;
    let task_iter = stmt.query_map(params![is_done], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            is_done: row.get(2)?,
            date: row.get(3)?,
        })
    })?;
    for task in task_iter {
        let task = task.unwrap();
        println!(
            "Task id: {:?}, Task name {:?}, Is_done: {:?}, date: {:?}",
            &task.id.unwrap(),
            &task.name,
            &task.is_done.unwrap(),
            &task.date
        );
    }
    Ok(())
}
fn list_all_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, is_done, date FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            is_done: row.get(2)?,
            date: row.get(3)?,
        })
    })?;
    for task in task_iter {
        let task = task.unwrap();
        println!(
            "Task id: {:?}, Task name {:?}, Is_done: {:?}, date: {:?}",
            &task.id.unwrap(),
            &task.name,
            &task.is_done.unwrap(),
            &task.date
        );
    }
    Ok(())
}
fn mark_done(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE task SET is_done = ?1 WHERE id = ?2", params![1, id])?;
    Ok(())
}

fn mark_undone(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE task SET is_done = ?1 WHERE id = ?2", params![0, id])?;
    Ok(())
}

fn remove_task(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM task WHERE id = ?1;", params![id])?;
    Ok(())
}

fn validate_date(date: &String) -> bool {
    let format = "%d-%m-%Y";
    match NaiveDate::parse_from_str(date.as_str(), format) {
        Ok(_) => true,
        Err(_) => false,
    }
}
