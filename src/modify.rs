use super::{todo, utility};

use std::env;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let todo = todo::parse_todo(todos.len(), &args.join(" ").to_string());
    utility::notify("ADDED", todos.len(), &todo.task);
    todos.push(todo);
    utility::save_to_file(&todos, env::var("TODOFILE")?)
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
    utility::notify("APPENDED", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_to_file(&todos, env::var("TODOFILE")?)
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
    utility::notify("PREPENDED", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn remove(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t rm IDX"));
    }
    let mut todos = utility::get_todos()?;
    for i in utility::parse_reversed_indices(&args)? {
        if i >= todos.len() {
            continue;
        }
        utility::notify("REMOVED", i, &todos[i].task);
        todos.remove(i);
    }
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn do_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    for i in utility::parse_reversed_indices(&args)? {
        if i >= todos.len() {
            continue;
        }
        let mut done_task = todos[i].clone();
        done_task.done = utility::get_formatted_date();
        utility::notify(
            "COMPLETE",
            i,
            &format!("{} {}", done_task.done, &todos[i].task),
        );
        dones.push(done_task.clone());
        todos.remove(i);
    }

    utility::save_to_file(&todos, env::var("TODOFILE")?)?;
    utility::save_to_file(&dones, env::var("DONEFILE")?)
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
    utility::notify(msg, todos.len(), &done.task);
    todos.push(done);
    dones.remove(idx);

    utility::save_to_file(&todos, env::var("TODOFILE")?)?;
    utility::save_to_file(&dones, env::var("DONEFILE")?)
}
