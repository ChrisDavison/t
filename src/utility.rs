use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use chrono::{Date, Duration, NaiveDate, TimeZone, Utc};

use super::todo::Todo;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn todo_filter<'a>(todos: &'a [Todo], filters: &'a [String]) -> impl Iterator<Item = &'a Todo> {
    let (bad, good): (Vec<_>, Vec<_>) = filters
        .iter()
        .map(|x| x.to_string())
        .partition(|x| x.starts_with('-'));
    todos.iter().filter(move |x| x.matches(&good, &bad))
}

pub fn notify<T: Display>(message: &str, task: T) {
    println!("{}: {}", message, task);
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
    let f = std::fs::File::open(filename).expect("Couldn't open file");
    let reader = BufReader::new(f);

    let mut todos = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let mut todo: Todo = line?.parse()?;
        todo.idx = idx;
        todos.push(todo);
    }
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
    fs::write(&filename, todo_str + "\n").expect("Couldn't write todos to file");
    // crate::utility::notify("SAVED FILE", &filename);
    Ok(())
}

pub fn parse_date(date: Option<&String>) -> Option<NaiveDate> {
    date.map(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .flatten()
}

pub fn date_today() -> Date<Utc> {
    if cfg!(test) {
        // Mon, September 13
        Utc.ymd(2021, 9, 13)
    } else {
        Utc::today()
    }
}

pub fn parse_date_string_relative(date: Date<Utc>, s: &str) -> String {
    let to_ymd = |date: Date<Utc>| date.format("%Y-%m-%d").to_string();
    match s.to_lowercase().as_str() {
        "today" => to_ymd(date_today()),
        "tomorrow" => to_ymd(date_today() + Duration::days(1)),
        "weekend" => to_ymd(iter_till_day_of_week(date, 6)),
        "monday" | "mon" => to_ymd(iter_till_day_of_week(date, 1)),
        "tuesday" | "tue" => to_ymd(iter_till_day_of_week(date, 2)),
        "wednesday" | "wed" => to_ymd(iter_till_day_of_week(date, 3)),
        "thursday" | "thu" => to_ymd(iter_till_day_of_week(date, 4)),
        "friday" | "fri" => to_ymd(iter_till_day_of_week(date, 5)),
        "saturday" | "sat" => to_ymd(iter_till_day_of_week(date, 6)),
        "sunday" | "sun" => to_ymd(iter_till_day_of_week(date, 7)),
        _ => date.to_string(),
    }
}

fn iter_till_day_of_week(date: Date<Utc>, day_of_week: u8) -> Date<Utc> {
    let mut date = date;
    let one_day = Duration::days(1);
    date = date + one_day;
    while date.format("%u").to_string().parse::<u8>().unwrap() != day_of_week {
        date = date + one_day;
    }
    date
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn iter_date_till_sat() {
        let mut now = date_today();
        let want = Utc.ymd(2021, 9, 18); // Sat, 18 September

        now = iter_till_day_of_week(now, 6);
        assert_eq!(now, want);
    }

    #[test]
    fn date_from_string() {
        let now = date_today();
        assert_eq!(parse_date_string_relative(now, "thursday"), "2021-09-16");
        assert_eq!(parse_date_string_relative(now, "tomorrow"), "2021-09-14");
        assert_eq!(parse_date_string_relative(now, "weekend"), "2021-09-18");
    }
}
