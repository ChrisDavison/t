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

pub fn filter_todos(todos: &[(usize, Todo)], args: &[String]) -> (Vec<(usize, Todo)>, Vec<String>) {
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
    let todos_filtered = todos
        .iter()
        .filter(|(_, x)| x.matches(&positives[..], &negatives[..]))
        .map(|(i, x)| (*i, x.clone()))
        .collect();
    (todos_filtered, raw_args)
}

fn parse_file<T: Into<String>>(filename: &T) -> Result<Vec<(usize, Todo)>>
where
    T: std::convert::AsRef<std::path::Path>,
{
    let mut f = std::fs::File::open(filename).expect("Couldn't open file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read contents of file");

    let todos: Vec<(usize, Todo)> = contents
        .lines()
        .enumerate()
        .map(|(i, x)| (i, x.parse().unwrap()))
        .collect();
    Ok(todos)
}

pub fn get_todos() -> Result<Vec<(usize, Todo)>> {
    let todofile = env::var("TODOFILE").map_err(|_| "TODOFILE env var not set")?;
    parse_file(&todofile)
}

pub fn get_dones() -> Result<Vec<(usize, Todo)>> {
    let donefile = env::var("DONEFILE").map_err(|_| "DONEFILE not set")?;
    parse_file(&donefile)
}

pub fn save_to_file(todos: &[(usize, Todo)], filename: String) -> Result<()> {
    let mut str_out = String::new();
    for (_, todo) in todos {
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
            "{} {} {} {}\n",
            todo.task, project_string, tag_string, kw_string,
        );
        str_out.push_str(&RE_SPC.replace(&todo_out_str, " ").to_string())
    }
    fs::write(filename, str_out).expect("Couldn't write todos to file");
    Ok(())
}

pub fn parse_reversed_indices(args: &[String]) -> Result<Vec<usize>> {
    let mut idx: Vec<usize> = args.iter().map(|x| x.parse()).map(|x| x.unwrap()).collect();
    idx.sort();
    idx.reverse();
    Ok(idx)
}
