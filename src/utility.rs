use std::env;
use std::fmt;
use std::fs;
use std::io::Read;

use regex::Regex;

use chrono::{DateTime, Utc};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

#[derive(Clone)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub priority: bool,
    pub date: String,
    pub done: String,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = if self.priority { " ! " } else { "" };
        let d = if self.date != "" {
            format!("{} ", self.date).to_string()
        } else {
            String::new()
        };
        let dd = if self.done != "" {
            format!("{} ", self.done).to_string()
        } else {
            String::new()
        };
        write!(f, "{:3}{:3}. {}{:11}{}", p, self.idx, dd, d, self.task)
    }
}

pub fn parse_todo(idx: usize, line: &str) -> Todo {
    let line = &line[2..];
    let (task, priority) = if line.starts_with("! ") {
        (&line[2..], true)
    } else {
        (line, false)
    };
    let re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    let (task, date) = if re_date.is_match(task) {
        (&task[11..], &task[..10])
    } else {
        (task, "")
    };
    Todo {
        idx,
        task: task.to_string(),
        priority,
        date: date.to_string(),
        done: "".to_string(),
    }
}

pub fn parse_done(idx: usize, line: &str) -> Todo {
    let line = &line[2..];
    let (done, task, priority) = if line.starts_with("! ") {
        (&line[2..12], &line[13..], true)
    } else {
        (&line[0..10], &line[11..], false)
    };
    let re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    let (task, date) = if re_date.is_match(task) {
        (&task[11..], &task[..10])
    } else {
        (task, "")
    };
    Todo {
        idx,
        task: task.to_string(),
        priority,
        date: date.to_string(),
        done: done.to_string(),
    }
}

pub fn todo_printable(t: &Todo) -> String {
    let p = if t.priority { "! " } else { "" };
    let d = if t.date != "" {
        format!("{} ", t.date).to_string()
    } else {
        "".to_string()
    };
    let msg = format!("- {}{}{}\n", p, d, t.task);
    let re_spc: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
    re_spc.replace(&msg, " ").to_string()
}

pub fn done_printable(d: &Todo) -> String {
    let p = if d.priority { "! " } else { "" };
    let dt = if d.date != "" {
        format!("{} ", d.date).to_string()
    } else {
        "".to_string()
    };
    let msg = format!("- {} {}{}{}\n", d.done, p, dt, d.task);
    let re_spc: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
    re_spc.replace(&msg, " ").to_string()
}

pub fn save_todos(todos: &[Todo]) -> Result<()> {
    save_to_file(&todos, env::var("TODOFILE")?, todo_printable)
}

pub fn save_dones(dones: &[Todo]) -> Result<()> {
    save_to_file(&dones, env::var("DONEFILE")?, done_printable)
}

pub fn save_to_file(
    todos: &[Todo],
    filename: String,
    formatter: fn(&Todo) -> String,
) -> Result<()> {
    let todos: String = todos.iter().map(formatter).collect();
    fs::write(filename, todos)?;
    Ok(())
}

pub fn get_todos() -> Result<Vec<Todo>> {
    let todofile = env::var("TODOFILE").expect("TODOFILE not defined");
    let mut f = std::fs::File::open(todofile)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .enumerate()
        .map(|(i, x)| parse_todo(i, x));
    Ok(todos.collect())
}

pub fn get_dones() -> Result<Vec<Todo>> {
    let filename = env::var("DONEFILE").expect("DONEFILE not defined");
    let mut f = std::fs::File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .enumerate()
        .map(|(i, x)| parse_done(i, &x));
    Ok(todos.collect())
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
