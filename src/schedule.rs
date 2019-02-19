use super::{todo, utility};

use std::io::{self, Write};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn unschedule(args: &[String]) -> Result<()> {
    let mut todos = utility::get::todos()?;
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t unschedule IDX")),
    };
    if idx >= todos.len() {
        return Err(From::from("Index out of bounds"));
    }
    todos[idx].date = "".to_string();
    println!("UNSCHEDULED {} {}", idx, &todos[idx].task);
    utility::save::todos(&todos)
}

pub fn today(args: &[String]) -> Result<()> {
    let mut todos: Vec<todo::Todo> = utility::get::todos()?;
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t today IDX")),
    };
    let t_str = utility::get_formatted_date().to_string();
    todos[idx].date = t_str;
    println!("TODAY {} {}", idx, todos[idx].task);
    utility::save::todos(&todos)
}

pub fn schedule(args: &[String]) -> Result<()> {
    let mut todos = utility::get::todos()?;
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
    utility::save::todos(&todos)
}
