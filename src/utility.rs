use std::env;
use std::fmt;
use std::fs;
use std::io::Read;

use super::view;

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
    let (task, date) = if view::re_date.is_match(task) {
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
    let (task, date) = if view::re_date.is_match(task) {
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

pub fn write_enumerated_todos(todos: &[Todo]) -> Result<()> {
    let todofile = env::var("TODOFILE")?;
    let todos: String = todos
        .iter()
        .map(|x| {
            let p = if x.priority { "! " } else { "" };
            let d = if x.date != "" {
                format!("{} ", x.date).to_string()
            } else {
                "".to_string()
            };
            let msg = format!("- {}{}{}\n", p, d, x.task);
            view::re_spc.replace(&msg, " ").to_string()
        })
        .collect();
    fs::write(todofile, todos)?;
    Ok(())
}

pub fn write_enumerated_dones(dones: &[Todo]) -> Result<()> {
    let filename = env::var("DONEFILE")?;
    let dones: String = dones
        .iter()
        .map(|x| {
            let p = if x.priority { "! " } else { "" };
            let d = if x.date != "" {
                format!("{} ", x.date).to_string()
            } else {
                "".to_string()
            };
            let msg = format!("- {} {}{}{}\n", x.done, p, d, x.task);
            view::re_spc.replace(&msg, " ").to_string()
        })
        .collect();
    fs::write(filename, dones)?;
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
    let positives: Vec<String> = args
        .iter()
        .filter(|&x| x.starts_with('+'))
        .map(|x| x[1..].to_owned())
        .collect();
    let negatives: Vec<String> = args
        .iter()
        .filter(|&x| x.starts_with('-'))
        .map(|x| x[1..].to_owned())
        .collect();
    let raw_args: Vec<String> = args
        .iter()
        .filter(|&x| !(x.starts_with('+') || x.starts_with('-')))
        .map(|x| x[1..].to_owned())
        .collect();
    let todos_positive: Vec<Todo> = todos
        .iter()
        .filter(|x| {
            positives
                .iter()
                .all(|y| case_insensitive_match(&x.task, &y))
        })
        .map(|x| x.to_owned())
        .collect();
    let todos_no_negative: Vec<Todo> = todos_positive
        .iter()
        .filter(|x| {
            !negatives
                .iter()
                .any(|y| case_insensitive_match(&x.task, &y))
        })
        .map(|x| x.to_owned())
        .collect();
    (todos_no_negative, raw_args)
}

pub fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}
