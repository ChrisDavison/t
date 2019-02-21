use super::{todo, utility};

use chrono::{Date, NaiveDate, Utc};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn get_datediff(t: &todo::Todo) -> Result<i64> {
    let date = t.date.clone();
    let now: Date<Utc> = Utc::now().date();
    let y = date[0..4].parse()?;
    let m = date[5..7].parse()?;
    let d = date[8..10].parse()?;
    let task_date = Date::from_utc(NaiveDate::from_ymd(y, m, d), *now.offset());
    Ok((now - task_date).num_days())
}

pub fn list(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    for t in todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn done(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for t in todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn due(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    let mut todos_with_datediff: Vec<(i64, todo::Todo)> = todos
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

pub fn no_date(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    for t in todos {
        if t.date == "" {
            println!("{}", t);
        }
    }
    Ok(())
}
