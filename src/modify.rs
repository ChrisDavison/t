use super::{utility, view};
use std::io::{self, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos(false)?;
    let msg = format!("- {}", args.join(" "));
    println!("CREATED {}", &msg[2..]);
    todos.push((todos.len(), msg));
    utility::write_enumerated_todos(&todos)
}

pub fn append(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    let new = format!("{} {}", todos[idx].1, msg);
    println!("APPENDED {}", &new[2..]);
    todos[idx] = (todos[idx].0, new);
    utility::write_enumerated_todos(&todos)
}

pub fn today(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let new = format!("{} due:{}", todos[idx].1, utility::get_formatted_date());
    println!("APPENDED {}", &new[2..]);
    todos[idx] = (todos[idx].0, new);
    utility::write_enumerated_todos(&todos)
}

pub fn remove(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t rm IDX"));
    }
    let mut todos = utility::get_todos(false)?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    println!("REMOVED {}", &todos[idx].1[2..]);
    todos.remove(idx);
    utility::write_enumerated_todos(&todos)
}

pub fn repeat_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let todos = utility::get_todos(false)?;
    let mut dones = utility::get_done()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    let dated_task = format!("{} done:{}", todos[idx].1, utility::get_formatted_date());
    println!("REPEATING {}", &todos[idx].1[2..]);
    dones.push((todos[idx].0, dated_task));

    utility::write_enumerated_dones(&dones)
}

pub fn do_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get_todos(false)?;
    let mut dones = utility::get_done()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    let dated_task = format!("{} done:{}", todos[idx].1, utility::get_formatted_date());
    println!("COMPLETE {}", &dated_task[2..]);
    dones.push((todos[idx].0, dated_task));
    todos.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub fn undo(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos(false)?;
    let mut dones = utility::get_done()?;
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
    let shortened = &dones[idx].1[..dones[idx].1.len() - 16];
    println!("{} {}", msg, &shortened[2..]);
    todos.push((dones[idx].0, shortened.to_owned()));
    dones.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub fn upgrade(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }

    if !todos[idx].1.starts_with("- ! ") {
        let msg = format!("- ! {}", &todos[idx].1[2..]);
        println!("UPGRADED {}", &msg[2..]);
        todos[idx] = (todos[idx].0, msg);
    }
    utility::write_enumerated_todos(&todos)
}

pub fn downgrade(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    if todos[idx].1.starts_with("- ! ") {
        let msg = format!("- {}", &todos[idx].1[4..]);
        println!("DOWNGRADED {}", &msg[2..]);
        todos[idx] = (todos[idx].0, msg);
    }
    utility::write_enumerated_todos(&todos)
}

pub fn clear_done() -> Result<()> {
    let filename = std::env::var("DONEFILE")?;
    println!("DONEFILE cleared");
    std::fs::write(filename, String::new())?;
    Ok(())
}

pub fn schedule(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
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
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let new = format!("{} due:{}", todos[idx].1, date);
    println!("SCHEDULED {}", &new[2..]);
    todos[idx] = (todos[idx].0, new);
    utility::write_enumerated_todos(&todos)
}

pub fn unschedule(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos(false)?;
    let idx: usize = args[0].parse()?;
    let (_, todo) = &todos[idx];
    println!("UNSCHEDULED {}", &todo[2..]);
    todos[idx] = (idx, view::re_due.replace(&todo, "").to_string());
    utility::write_enumerated_todos(&todos)
}
