use std::env;
use std::fs;
use std::io::Read;

use chrono::{DateTime, Utc};

use super::todo::{Todo, RE_SPC};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn notify(message: &str, index: usize, task: &str) {
    println!("{}: {:4}. {}", message, index, task);
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
        match arg.chars().next().expect("Couldn't read char of query arg") {
            '+' => positives.push(arg[1..].to_owned()),
            '-' => negatives.push(arg[1..].to_owned()),
            _ => raw_args.push(arg.to_owned()),
        }
    }
    let todos_filtered = todos
        .iter()
        .filter(|x| x.matches(&positives[..], &negatives[..]))
        .cloned()
        .collect();
    (todos_filtered, raw_args)
}

fn parse_file<T: Into<String>>(filename: &T) -> Result<Vec<Todo>>
where
    T: std::convert::AsRef<std::path::Path>,
{
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
    parse_file(&todofile)
}

pub fn get_dones() -> Result<Vec<Todo>> {
    let donefile = env::var("DONEFILE").map_err(|_| "DONEFILE not set")?;
    parse_file(&donefile)
}

pub fn save_to_file(todos: &[Todo], filename: String) -> Result<()> {
    let mut str_out = String::new();
    for todo in todos {
        let kw_string = todo
            .kws
            .iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join(" ");

        let project_string = todo
            .projects
            .iter()
            .map(|p| format!("@{}", p))
            .collect::<Vec<String>>()
            .join(" ");

        let tag_string = todo
            .tags
            .iter()
            .map(|p| format!("+{}", p))
            .collect::<Vec<String>>()
            .join(" ");
        let todo_out_str = &format!(
            "{:4}. {} {} {} {}\n",
            todo.idx, todo.task, project_string, tag_string, kw_string,
        );
        str_out.push_str(&RE_SPC.replace(todo_out_str, " ").to_string())
    }
    fs::write(filename, str_out).expect("Couldn't write todos to file");
    Ok(())
}

pub fn parse_reversed_indices(idxs: &[usize]) -> Result<Vec<usize>> {
    let mut idx = idxs.to_vec();
    idx.sort_unstable();
    idx.reverse();
    Ok(idx)
}
