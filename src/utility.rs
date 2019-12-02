use std::env;
use std::fs;
use std::io::Read;

use chrono::{DateTime, Utc};

use super::todo::{self, Todo};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn notify(m: &str, i: usize, t: &str) {
    println!("{} :: {} :: {}", m, i, t);
}

pub fn get_formatted_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d").to_string()
}

pub fn filter_todos(todos: &[Todo], args: &[String]) -> (Vec<Todo>, Vec<String>) {
    let mut positives: Vec<String> = Vec::new();
    let mut negatives: Vec<String> = Vec::new();
    let mut raw_args: Vec<String> = Vec::new();
    for arg in args {
        match arg.chars().nth(0).expect("Couldn't read char of query arg") {
            '+' => positives.push(arg[1..].to_owned()),
            '-' => negatives.push(arg[1..].to_owned()),
            _ => raw_args.push(arg.to_owned()),
        }
    }
    let mut todos_filtered = Vec::new();
    for todo in todos {
        let has_all_pos = positives
            .iter()
            .all(|y| case_insensitive_match(&todo.task, &y));
        let has_no_neg = !negatives
            .iter()
            .any(|y| case_insensitive_match(&todo.task, &y));
        if has_all_pos && has_no_neg {
            todos_filtered.push(todo.to_owned());
        }
    }
    (todos_filtered, raw_args)
}

pub fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

fn parse_file<T: Into<String>>(filename: &T, parser: fn(usize, &str) -> Todo) -> Result<Vec<Todo>>
where
    T: std::convert::AsRef<std::path::Path>,
{
    let mut f = std::fs::File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .enumerate()
        .map(|(i, x)| parser(i, &x));
    Ok(todos.collect())
}

pub fn get_todos() -> Result<Vec<Todo>> {
    parse_file(&env::var("TODOFILE")?, todo::parse_todo)
}

pub fn get_dones() -> Result<Vec<Todo>> {
    parse_file(&env::var("DONEFILE")?, todo::parse_done)
}

pub fn save_to_file(todos: &[Todo], filename: String) -> Result<()> {
    let todos: String = todos.iter().map(todo::printable).collect();
    fs::write(filename, todos)?;
    Ok(())
}

pub fn parse_reversed_indices(args: &[String]) -> Result<Vec<usize>> {
    let mut idx: Vec<usize> = args.iter().map(|x| x.parse()).map(|x| x.unwrap()).collect();
    idx.sort();
    idx.reverse();
    Ok(idx)
}
