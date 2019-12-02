use super::{todo, utility};

use chrono::{Date, NaiveDate, Utc};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn days_overdue(t: &todo::Todo) -> i64 {
    let now: Date<Utc> = Utc::now().date();
    let naive = NaiveDate::parse_from_str(&t.date, "%Y-%m-%d").expect("Couldn't parse date");
    let task_date = Date::from_utc(naive, *now.offset());
    (now - task_date).num_days()
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
    for todo in todos {
        println!("{}", todo);
    }
    Ok(())
}

pub fn due(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    let mut datediffed_todos: Vec<(i64, todo::Todo)> = todos
        .iter()
        .filter(|x| x.date != "")
        .map(|x| (days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(&datediff1));
    for (_diff, t) in datediffed_todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn no_date(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    let undated_todos = todos.iter().filter(|x| x.date == "");
    for t in undated_todos {
        println!("{}", t);
    }
    Ok(())
}
