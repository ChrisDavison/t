use super::{todo, utility};

use chrono::{Date, NaiveDate, Utc};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn days_overdue(t: &todo::Todo) -> i64 {
    let now: Date<Utc> = Utc::now().date();
    let naive =
        NaiveDate::parse_from_str(t.kws.get("due").unwrap_or(&String::from("")), "%Y-%m-%d")
            .expect("Couldn't parse date");
    let task_date = Date::from_utc(naive, *now.offset());
    (now - task_date).num_days()
}

pub fn list(todos: &[todo::Todo], filters: &[String]) -> Result<()> {
    let (mut todos, _args) = utility::filter_todos(todos, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| !x.pri.is_empty()) {
        println!("{}", todo);
    }
    for todo in todos.iter().filter(|x| x.pri.is_empty()) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn list_priority(todos: &[todo::Todo], filters: &[String]) -> Result<()> {
    let (mut todos, _args) = utility::filter_todos(todos, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| !x.pri.is_empty()) {
        println!("{}", todo);
    }
    Ok(())
}

pub fn done(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (mut todos, _) = utility::filter_todos(todos, args);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| !x.pri.is_empty()) {
        println!("{}", todo);
    }
    for todo in todos.iter().filter(|x| x.pri.is_empty()) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn due(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(todos, args);

    let mut datediffed_todos: Vec<(i64, todo::Todo)> = todos
        .iter()
        .filter(|x| x.kws.contains_key("due"))
        .map(|x| (days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(datediff1));
    for (_, t) in datediffed_todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn no_date(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(todos, args);
    let undated_todos: Vec<_> = todos
        .iter()
        .filter(|x| !x.kws.contains_key("due"))
        .collect();
    for todo in undated_todos.iter().filter(|x| !x.pri.is_empty()) {
        println!("{}", todo);
    }
    for todo in undated_todos.iter().filter(|x| x.pri.is_empty()) {
        println!("{}", todo);
    }

    Ok(())
}
