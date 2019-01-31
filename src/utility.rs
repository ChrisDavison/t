use std::env;
use std::fs;
use std::io::Read;

use chrono::{DateTime, Utc};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn write_enumerated_todos(todos: &[(usize, String)]) -> Result<()> {
    let todofile = env::var("TODOFILE")?;
    let todos: String = todos.iter().map(|(_, x)| format!("{}\n", x)).collect();
    fs::write(todofile, todos)?;
    Ok(())
}

pub fn write_enumerated_dones(dones: &[(usize, String)]) -> Result<()> {
    let filename = env::var("DONEFILE")?;
    let dones: String = dones.iter().map(|(_, x)| format!("{}\n", x)).collect();
    fs::write(filename, dones)?;
    Ok(())
}

pub fn get_todos(organised: bool) -> Result<Vec<(usize, String)>> {
    let todofile = env::var("TODOFILE").expect("TODOFILE not defined");
    let mut f = std::fs::File::open(todofile)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .map(|x| x.to_owned())
        .enumerate();
    if organised {
        Ok(todos
            .clone()
            .filter(|(_, x)| x.starts_with("- !"))
            .chain(todos.filter(|(_, x)| !x.starts_with("- !")))
            .collect())
    } else {
        Ok(todos.collect())
    }
}

pub fn get_done() -> Result<Vec<(usize, String)>> {
    let filename = env::var("DONEFILE").expect("DONEFILE not defined");
    let mut f = std::fs::File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .map(|x| x.to_owned())
        .enumerate();
    Ok(todos.collect())
}

pub fn print_todo_filename() -> Result<()> {
    let todofile = env::var("TODOFILE")?;
    println!("{}", todofile);
    Ok(())
}

pub fn get_formatted_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d").to_string()
}

pub fn check_for_blank_files() -> Result<()> {
    let todos = get_todos(false)?;
    if todos.is_empty() {
        println!("TODOFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    let dones = get_done()?;
    if dones.is_empty() {
        println!("DONEFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    Ok(())
}
