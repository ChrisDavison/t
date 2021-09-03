use super::utility;

use std::env;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn unschedule(args: &[usize]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    for i in utility::parse_reversed_indices(args)? {
        if i >= todos.len() {
            continue;
        }
        todos[i].kws.remove("due");
        utility::notify("UNSCHEDULED", i, &todos[i].task);
    }
    // utility::notify("UNSCHEDULED", idx, &todos[idx].task);
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn today(args: &[usize]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let t_str = utility::get_formatted_date();
    for i in utility::parse_reversed_indices(args)? {
        todos[i].kws.insert("due".to_string(), t_str.clone());
        utility::notify("TODAY", i, &todos[i].task);
    }
    utility::save_to_file(&todos[..], env::var("TODOFILE")?)
}

pub fn schedule(args: &[usize], date: &str) -> Result<()> {
    let mut todos = utility::get_todos()?;
    for i in utility::parse_reversed_indices(args)? {
        todos[i].kws.insert("due".to_string(), date.to_string());
        utility::notify("SCHEDULED", i, &todos[i].task);
    }
    utility::save_to_file(&todos[..], env::var("TODOFILE")?)
}
