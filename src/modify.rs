use super::{utility, view};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let msg = args.join(" ").to_string();
    let (task, priority) = if msg.starts_with("! ") {
        (msg[2..].to_string(), true)
    } else {
        (msg.to_string(), false)
    };
    let (task, date) = if view::re_date.is_match(&task) {
        (&task[11..], &task[..10])
    } else {
        (&task[..], "")
    };
    let todo = utility::Todo {
        idx: todos.len(),
        task: task.to_string(),
        priority: priority,
        date: date.to_string(),
    };
    println!("ADDED {}", todo.task);
    todos.push(todo);
    utility::write_enumerated_todos(&todos)
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
    println!("APPENDED {}", &new.task);
    todos[idx] = new.clone();
    utility::write_enumerated_todos(&todos)
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
    println!("PREPENDED {}", &new.task);
    todos[idx] = new.clone();
    utility::write_enumerated_todos(&todos)
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
    println!("REMOVED {}", &todos[idx].task);
    todos.remove(idx);
    utility::write_enumerated_todos(&todos)
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
    done_task.task = format!("{} done:{}", done_task.task, utility::get_formatted_date());
    println!("COMPLETE {}", &todos[idx].task);
    dones.push(done_task.clone());
    todos.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub fn undo(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    let mut msg = "UNDONE";
    let idx = if args.is_empty() {
        msg = "UNDONE LAST";
        dones.len() - 1
    } else {
        args[0].parse()?
    };
    if idx >= dones.len() {
        return Err(From::from("IDX must be within range of num done"));
    }
    let mut shortened = dones[idx].clone();
    shortened.task = shortened.task[..shortened.task.len() - 16].to_string();
    shortened.idx = todos.len();
    println!("{} {}", msg, &shortened.task);
    todos.push(shortened);
    dones.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub mod prioritise {
    use super::*;

    pub fn upgrade(args: &[String]) -> Result<()> {
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t up IDX")),
        };
        let mut todos = utility::get_todos()?;
        if todos.len() < idx {
            return Err(From::from(format!(
                "IDX must be < {} (number of tasks)",
                todos.len()
            )));
        }
        todos[idx].priority = true;
        println!("UPGRADED {}", todos[idx].task);
        utility::write_enumerated_todos(&todos)
    }

    pub fn downgrade(args: &[String]) -> Result<()> {
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t down IDX")),
        };
        let mut todos = utility::get_todos()?;
        if todos.len() < idx {
            return Err(From::from(format!(
                "IDX must be < {} (number of tasks)",
                todos.len()
            )));
        }
        todos[idx].priority = false;
        println!("DOWNGRADED {}", todos[idx].task);
        utility::write_enumerated_todos(&todos)
    }
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
        println!("UNSCHEDULED {}", &todos[idx].task);
        utility::write_enumerated_todos(&todos)
    }

    pub fn today(args: &[String]) -> Result<()> {
        let mut todos: Vec<utility::Todo> = utility::get_todos()?;
        let idx: usize = match args.get(0) {
            Some(i) => i.parse()?,
            None => return Err(From::from("usage: t today IDX")),
        };
        let t_str = format!("{}", utility::get_formatted_date());
        todos[idx].date = t_str;
        println!("TODAY {}", todos[idx].task);
        utility::write_enumerated_todos(&todos)
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
        let t_str = format!("{}", date);
        todos[idx].date = t_str;
        println!("SCHEDULED {}", todos[idx].task);
        utility::write_enumerated_todos(&todos)
    }
}
