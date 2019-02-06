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

pub fn parse_args() -> (String, Vec<String>, Vec<String>, Vec<String>) {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    // let cmd: String = env::args().skip(1).take(1).collect();
    // let args: Vec<String> = env::args().skip(2).collect();
    let positives: Vec<String> = raw_args
        .iter()
        .filter(|&x| x.starts_with('+'))
        .map(|x| x[1..].to_owned())
        .collect();
    let negatives: Vec<String> = raw_args
        .iter()
        .filter(|&x| x.starts_with('-'))
        .map(|x| x[1..].to_owned())
        .collect();
    let args_final: Vec<String> = raw_args
        .iter()
        .filter(|&x| !x.starts_with('+') && !x.starts_with('-'))
        .map(|x| x.to_owned())
        .collect();
    (args_final.iter().take(1).map(|x| x.to_owned()).collect(), positives, negatives, args_final.iter().skip(1).map(|x| x.to_owned()).collect())
}

pub fn filter_todos(
    todos: &[(usize, String)],
    positives: &[String],
    negatives: &[String],
) -> Vec<(usize, String)> {
    let todos_positive: Vec<(usize, String)> = todos
        .iter()
        .filter(|&(_, x)| positives.iter().all(|y| x.contains(y)))
        .map(|(i, x)| (i.to_owned(), x.to_owned()))
        .collect();
    let todos_no_negative: Vec<(usize, String)> = todos_positive
        .iter()
        .filter(|&(_, x)| !negatives.iter().any(|y| x.contains(y)))
        .map(|(i, x)| (i.to_owned(), x.to_owned()))
        .collect();
    todos_no_negative
}
