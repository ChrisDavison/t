use regex::Regex;

use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let msg = args.join(" ").to_string();
    let (task, priority) = if msg.starts_with("! ") {
        (msg[2..].to_string(), true)
    } else {
        (msg.to_string(), false)
    };
    let re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    let (task, date) = if re_date.is_match(&task) {
        (&task[11..], &task[..10])
    } else {
        (&task[..], "")
    };
    let todo = utility::Todo {
        idx: todos.len(),
        task: task.to_string(),
        priority,
        date: date.to_string(),
        done: String::new(),
    };
    println!("ADDED {} {}", todos.len(), todo.task);
    todos.push(todo);
    utility::save_todos(&todos)
}

pub fn append(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t app IDX TEXT...")),
    };
    let mut todos = utility::get_todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", new.task, msg);
    println!("APPENDED {} {}", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_todos(&todos)
}

pub fn prepend(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t app IDX TEXT...")),
    };
    let mut todos = utility::get_todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", msg, new.task);
    println!("PREPENDED {} {}", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_todos(&todos)
}

pub fn remove(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t rm IDX"));
    }
    let mut todos = utility::get_todos()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    println!("REMOVED {} {}", idx, &todos[idx].task);
    todos.remove(idx);
    utility::save_todos(&todos)
}

pub fn do_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    let mut done_task = todos[idx].clone();
    done_task.done = utility::get_formatted_date();
    println!("COMPLETE {} {} {}", idx, done_task.done, &todos[idx].task);
    dones.push(done_task.clone());
    todos.remove(idx);

    utility::save_todos(&todos)?;
    utility::save_dones(&dones)
}

pub fn undo(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    let (idx, msg) = if args.is_empty() {
        (dones.len() - 1, "UNDONE LAST")
    } else {
        (args[0].parse()?, "UNDONE")
    };
    if idx >= dones.len() {
        return Err(From::from("IDX must be within range of num done"));
    }
    let mut done = dones[idx].clone();
    done.idx = todos.len();
    println!("{} {} {}", msg, todos.len(), &done.task);
    todos.push(done);
    dones.remove(idx);

    utility::save_todos(&todos)?;
    utility::save_dones(&dones)
}

fn change_priority(args: &[String], new: bool) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX argument")),
    };
    let mut todos = utility::get_todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    todos[idx].priority = new;
    let msg = if new { "UPGRADED" } else { "DOWNGRADED" };
    println!("{} {} {}", msg, idx, todos[idx].task);
    utility::save_todos(&todos)
}

pub fn upgrade(args: &[String]) -> Result<()> {
    change_priority(&args, true)
}

pub fn downgrade(args: &[String]) -> Result<()> {
    change_priority(&args, false)
}

pub mod schedule {
    use super::*;
    use std::io::{self, Write};

    pub fn unschedule(args: &[String]) -> Result<()> {
        let mut todos = utility::get_todos()?;
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t unschedule IDX")),
        };
        if idx >= todos.len() {
            return Err(From::from("Index out of bounds"));
        }
        todos[idx].date = "".to_string();
        println!("UNSCHEDULED {} {}", idx, &todos[idx].task);
        utility::save_todos(&todos)
    }

    pub fn today(args: &[String]) -> Result<()> {
        let mut todos: Vec<utility::Todo> = utility::get_todos()?;
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t today IDX")),
        };
        let t_str = utility::get_formatted_date().to_string();
        todos[idx].date = t_str;
        println!("TODAY {} {}", idx, todos[idx].task);
        utility::save_todos(&todos)
    }

    pub fn schedule(args: &[String]) -> Result<()> {
        let mut todos = utility::get_todos()?;
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t schedule IDX DATE")),
        };
        let date: String = match args.get(1) {
            Some(i) => i.to_owned(),
            None => {
                let mut date = String::new();
                print!("Date: ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut date)?;
                date
            }
        };
        let t_str = date.to_string();
        todos[idx].date = t_str;
        println!("SCHEDULED {} {}", idx, todos[idx].task);
        utility::save_todos(&todos)
    }
}
