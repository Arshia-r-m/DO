use rusqlite::{params, Connection, Result};
use std::env;

#[derive(Debug)]
struct Task {
    id: Option<i32>,
    name: String,
    data: Option<String>,
    is_done: Option<bool>,
    date: String,
}

fn main() {
    let conn = match database_connection() {
        Ok(conn) => conn,
        Err(e) => {
            panic!()
        }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Help!");
    } else {
        let com: &str = &args[1];
        let mut res: Result<()>;
        match com {
            "list" => {
                if args.len() == 2 {
                    res = list_all_tasks(&conn);
                } else {
                    let is_done = match args[2].as_str() {
                        "done" => 1,
                        "not-done" => 0,
                        _ => -1,
                    };
                    if is_done == -1 {
                        println!("Invalid input for tasks")
                    } else {
                        res = list_tasks(&conn, is_done);
                    }
                }
            }
            "add" => {
                if args.len() > 2 && args.len() < 5 {
                    let task = Task {
                        id: None,
                        name: args[2].clone(),
                        data: None,
                        is_done: None,
                        date: args[3].clone(),
                    };

                    res = add_task(&conn, task);
                } else {
                    println!("Please provide information for your task!");
                }
            }
            "done" => res = mark_done(&conn, args[2].parse::<i32>().unwrap()),
            _ => println!("Wrongg"),
        }
    }
}

fn database_connection() -> Result<Connection> {
    let conn = Connection::open("DO.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
                id    INTEGER PRIMARY KEY AUTOINCREMENT,
                name  TEXT NOT NULL,
                data  TEXT DEFAULT NULL,
                is_done  INTEGER DEFAULT 0,
                date  TEXT DEFAULT NULL
            )",
        [], // Empty params list, no parameters in the query
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
    let mut stmt = conn.prepare("SELECT id, name, date FROM task WHERE is_done = ?1")?;
    let task_iter = stmt.query_map(params![is_done], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            data: None,
            is_done: None,
            date: row.get(2)?,
        })
    })?;
    for task in task_iter {
        let task = task.unwrap();
        println!(
            "Task id: {:?}, Task name {:?}, date: {:?}",
            &task.id, &task.name, &task.date
        );
    }
    Ok(())
}
fn list_all_tasks(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, date FROM task")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            data: None,
            is_done: None,
            date: row.get(2)?,
        })
    })?;
    for task in task_iter {
        let task = task.unwrap();
        println!(
            "Task id: {:?}, Task name {:?}, date: {:?}",
            &task.id, &task.name, &task.date
        );
    }
    Ok(())
}
fn mark_done(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("UPDATE task SET is_done = ?1 WHERE id = ?2", params![1, id])?;
    Ok(())
}
