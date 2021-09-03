use super::{todo, utility};

use std::env;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn add(text: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let mut todo: todo::Todo = text.join(" ").parse()?;
    utility::notify("ADDED", todos.len(), &todo.task);
    todo.idx = todos.len();
    todos.push(todo);
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn append(idx: usize, text: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = text.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", new.task, msg);
    utility::notify("APPENDED", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn prepend(idx: usize, text: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = text.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", msg, new.task);
    utility::notify("PREPENDED", idx, &new.task);
    todos[idx] = new.clone();
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn remove(idx: &[usize]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    for i in utility::parse_reversed_indices(idx)? {
        if i >= todos.len() {
            continue;
        }
        utility::notify("REMOVED", i, &todos[i].task);
        todos.remove(i);
    }
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn do_task(args: &[usize]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    for i in utility::parse_reversed_indices(args)? {
        if i >= todos.len() {
            continue;
        }
        let mut done_task = todos[i].clone();
        done_task
            .kws
            .insert("done".to_string(), utility::get_formatted_date());
        utility::notify(
            "COMPLETE",
            i,
            &format!("{} {}", done_task.kws["done"], &todos[i].task),
        );
        dones.push(done_task.clone());
        todos.remove(i);
    }

    utility::save_to_file(&todos, env::var("TODOFILE")?)?;
    utility::save_to_file(&dones, env::var("DONEFILE")?)
}

pub fn undo(args: &[usize]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let mut dones = utility::get_dones()?;
    for i in utility::parse_reversed_indices(args)? {
        if i >= dones.len() {
            return Err(From::from("IDX must be within range of num done"));
        }
        let done = dones[i].clone();
        utility::notify("UNDONE", todos.len(), &done.task);
        todos.push(done);
        dones.remove(i);
    }

    utility::save_to_file(&todos, env::var("TODOFILE")?)?;
    utility::save_to_file(&dones, env::var("DONEFILE")?)
}
