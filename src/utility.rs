use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use chrono::Utc;

use super::todo::Todo;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn notify(message: &str, index: usize, task: &str) {
    println!("{}: {:4}. {}", message, index, task);
}

#[cfg(test)]
pub fn get_formatted_date() -> String {
    "2021-01-01".to_string()
}

#[cfg(not(test))]
pub fn get_formatted_date() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

pub fn filter_todos(todos: &[Todo], filters: &[String]) -> Vec<Todo> {
    let (negatives, positives): (Vec<_>, Vec<_>) = filters
        .iter()
        .map(|x| x.to_string())
        .partition(|x| x.starts_with('-'));
    todos
        .iter()
        .filter(|x| x.matches(&positives, &negatives))
        .cloned()
        .collect()
}

fn parse_file(filename: &Path) -> Result<Vec<Todo>> {
    let mut f = std::fs::File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read contents of file");

    let todos: Vec<Todo> = contents
        .lines()
        .enumerate()
        .map(|(i, x)| {
            let mut todo: Todo = x.parse().unwrap();
            todo.idx = i;
            todo
        })
        .collect();
    Ok(todos)
}

pub fn get_todos() -> Result<Vec<Todo>> {
    let todofile = env::var("TODOFILE").map_err(|_| "TODOFILE env var not set")?;
    parse_file(&PathBuf::from(todofile))
}

pub fn get_dones() -> Result<Vec<Todo>> {
    let donefile = env::var("DONEFILE").map_err(|_| "DONEFILE not set")?;
    parse_file(&PathBuf::from(donefile))
}

pub fn save_to_file(todos: &[Todo], filename: String) -> Result<()> {
    let todo_str = todos
        .iter()
        .map(|x| x.format_for_save())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(filename, todo_str + "\n").expect("Couldn't write todos to file");
    Ok(())
}

pub fn parse_reversed_indices(idxs: &mut Vec<usize>) -> Result<Vec<usize>> {
    let mut idx = idxs.to_vec();
    idx.sort_unstable();
    idx.reverse();
    Ok(idx)
}

pub fn parse_date(date: Option<&String>) -> Option<NaiveDate> {
    date.map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .flatten()
}
