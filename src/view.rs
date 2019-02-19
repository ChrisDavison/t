use super::utility::{self, Todo};

use chrono::{Date, NaiveDate, Utc};
use regex::Regex;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

lazy_static! {
    pub static ref re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    pub static ref re_spc: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
}

pub fn get_datediff(t: &Todo) -> Result<i64> {
    let date = t.date.clone();
    let now: Date<Utc> = Utc::now().date();
    let y = date[0..4].parse()?;
    let m = date[5..7].parse()?;
    let d = date[8..10].parse()?;
    let task_date = Date::from_utc(NaiveDate::from_ymd(y, m, d), *now.offset());
    Ok((now - task_date).num_days())
}

pub fn list(todos: &[Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    for t in todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn list_priorities(todos: &[Todo], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for t in todos {
        if t.priority {
            println!("{}", t);
        }
    }
    Ok(())
}

pub fn done(todos: &[Todo], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for t in todos {
        println!("{}", t);
    }
    Ok(())
}

pub mod dated {
    use super::*;

    pub fn due(todos: &[Todo], args: &[String]) -> Result<()> {
        let (todos, _args) = utility::filter_todos(&todos, &args);
        let mut todos_with_datediff: Vec<(i64, Todo)> = todos
            .iter()
            .filter(|x| x.date != "")
            .map(|x| (get_datediff(x).unwrap(), x.to_owned()))
            .collect();
        todos_with_datediff.sort_by(|a, b| b.0.cmp(&a.0));
        for (_diff, t) in todos_with_datediff {
            println!("{}", t);
        }
        Ok(())
    }

    pub fn mit(todos: &[Todo], args: &[String]) -> Result<()> {
        let (todos, _args) = utility::filter_todos(&todos, &args);
        let mut todos_with_datediff: Vec<(i64, Todo)> = todos
            .iter()
            .filter(|x| x.date != "" && x.priority)
            .map(|x| (get_datediff(x).unwrap(), x.to_owned()))
            .filter(|(i, _x)| *i >= 0)
            .collect();
        todos_with_datediff.sort_by(|a, b| b.0.cmp(&a.0));
        for (_diff, t) in todos_with_datediff {
            println!("{}", t);
        }
        Ok(())
    }

    pub fn no_date(todos: &[Todo], args: &[String]) -> Result<()> {
        let (todos, _args) = utility::filter_todos(&todos, &args);
        for t in todos {
            if t.date == "" {
                println!("{}", t);
            }
        }
        Ok(())
    }
}
