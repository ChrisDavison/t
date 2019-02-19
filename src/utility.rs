use std::env;
use std::fs;

use chrono::{DateTime, Utc};

use super::todo::{self, Todo};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

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

pub mod get {
    use std::io::Read;

    use super::super::todo;
    use super::*;

    pub fn todos() -> Result<Vec<Todo>> {
        let todofile = env::var("TODOFILE").expect("TODOFILE not defined");
        let mut f = std::fs::File::open(todofile)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let todos = contents
            .lines()
            .filter(|x| x.starts_with("- "))
            .enumerate()
            .map(|(i, x)| todo::parse_todo(i, &x[2..]));
        Ok(todos.collect())
    }

    pub fn dones() -> Result<Vec<Todo>> {
        let filename = env::var("DONEFILE").expect("DONEFILE not defined");
        let mut f = std::fs::File::open(filename)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let todos = contents
            .lines()
            .filter(|x| x.starts_with("- "))
            .enumerate()
            .map(|(i, x)| todo::parse_done(i, &x));
        Ok(todos.collect())
    }
}

pub mod save {
    use super::*;

    pub fn todos(todos: &[Todo]) -> Result<()> {
        to_file(&todos, env::var("TODOFILE")?)
    }

    pub fn dones(dones: &[Todo]) -> Result<()> {
        to_file(&dones, env::var("DONEFILE")?)
    }

    fn to_file(todos: &[Todo], filename: String) -> String) -> Result<()> {
        let todos: String = todos.iter().map(todo::printable).collect();
        fs::write(filename, todos)?;
        Ok(())
    }
}
