use super::utility;

use std::env;
use std::io::{self, Write};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn unschedule(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    for i in utility::parse_reversed_indices(&args)? {
        if i >= todos.len() {
            continue;
        }
        todos[i].1.kws.remove("due");
        utility::notify("UNSCHEDULED", i, &todos[i].1.task);
    }
    // utility::notify("UNSCHEDULED", idx, &todos[idx].task);
    utility::save_to_file(&todos, env::var("TODOFILE")?)
}

pub fn today(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos()?;
    let t_str = utility::get_formatted_date().to_string();
    for i in utility::parse_reversed_indices(&args)? {
        todos[i].1.kws.insert("due".to_string(), t_str.clone());
        utility::notify("TODAY", i, &todos[i].1.task);
    }
    utility::save_to_file(&todos[..], env::var("TODOFILE")?)
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
    todos[idx].1.kws.insert("due".to_string(), t_str);
    utility::notify("SCHEDULED", idx, &todos[idx].1.task);
    utility::save_to_file(&todos[..], env::var("TODOFILE")?)
}
