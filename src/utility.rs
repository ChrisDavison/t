use std::env;
use std::fmt;
use std::fs;
use std::io::Read;

use super::view;

use chrono::{DateTime, Utc};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub priority: bool,
    pub date: String,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = if self.priority { "! " } else { "" };
        let d = if self.date != "" {
            format!("{} ", self.date).to_string()
        } else {
            "".to_string()
        };
        write!(f, "{:3}. {}{}{}", self.idx, p, d, self.task)
    }
}

fn parse_todo(idx: usize, line: &str) -> Todo {
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
        idx: idx,
        task: task.to_string(),
        priority: priority,
        date: date.to_string(),
    }
}

pub fn get_todos_vec() -> Result<Vec<Todo>> {
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

pub fn write_enumerated_todos(todos: &[(usize, String)]) -> Result<()> {
    let todofile = env::var("TODOFILE")?;
    let todos: String = todos
        .iter()
        .map(|(_, x)| {
            let msg = format!("{}\n", x);
            view::re_spc.replace(&msg, " ").to_string()
        })
        .collect();
    fs::write(todofile, todos)?;
    Ok(())
}

pub fn write_enumerated_dones(dones: &[(usize, String)]) -> Result<()> {
    let filename = env::var("DONEFILE")?;
    let dones: String = dones
        .iter()
        .map(|(_, x)| {
            let msg = format!("{}\n", x);
            view::re_spc.replace(&msg, " ").to_string()
        })
        .collect();
    fs::write(filename, dones)?;
    Ok(())
}

pub fn get_todos() -> Result<Vec<(usize, String)>> {
    let todofile = env::var("TODOFILE").expect("TODOFILE not defined");
    let mut f = std::fs::File::open(todofile)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let todos = contents
        .lines()
        .filter(|x| x.starts_with("- "))
        .map(|x| x.to_owned())
        .enumerate();
    Ok(todos.collect())
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
    let todos = get_todos()?;
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

pub fn filter_todos(
    todos: &[(usize, String)],
    args: &[String],
) -> (Vec<(usize, String)>, Vec<String>) {
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
    let todos_positive: Vec<(usize, String)> = todos
        .iter()
        .filter(|&(_, x)| positives.iter().all(|y| case_insensitive_match(&x, &y)))
        .map(|(i, x)| (i.to_owned(), x.to_owned()))
        .collect();
    let todos_no_negative: Vec<(usize, String)> = todos_positive
        .iter()
        .filter(|&(_, x)| !negatives.iter().any(|y| case_insensitive_match(&x, &y)))
        .map(|(i, x)| (i.to_owned(), x.to_owned()))
        .collect();
    (todos_no_negative, raw_args)
}

pub fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}
