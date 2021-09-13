use super::{todo, utility};

use chrono::{Date, NaiveDate, Utc};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn days_overdue(t: &todo::Todo) -> i64 {
    let now: Date<Utc> = Utc::now().date();
    let naive = NaiveDate::parse_from_str(
        t.due_date.as_ref().unwrap_or(&String::from("")).as_ref(),
        "%Y-%m-%d",
    )
    .expect("Couldn't parse date");
    let task_date = Date::from_utc(naive, *now.offset());
    (now - task_date).num_days()
}

pub fn list(todos: &[todo::Todo], filters: &[String]) -> Result<()> {
    let mut todos = utility::filter_todos(todos, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    for todo in todos.iter().filter(|x| x.pri.is_none()) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn list_priority(todos: &[todo::Todo], filters: &[String]) -> Result<()> {
    dbg!(&filters);
    let mut todos = utility::filter_todos(todos, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    Ok(())
}

pub fn done(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let mut todos = utility::filter_todos(todos, args);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    for todo in todos.iter().filter(|x| x.pri.is_none()) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn due(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let todos = utility::filter_todos(todos, args);

    let mut datediffed_todos: Vec<(i64, todo::Todo)> = todos
        .iter()
        .filter(|x| x.due_date.is_some())
        .map(|x| (days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(datediff1));
    for (_, t) in datediffed_todos {
        println!("{}", t);
    }
    Ok(())
}

pub fn no_date(todos: &[todo::Todo], args: &[String]) -> Result<()> {
    let todos = utility::filter_todos(todos, args);
    let undated_todos: Vec<_> = todos.iter().filter(|x| x.due_date.is_none()).collect();
    for &todo in undated_todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    for todo in undated_todos.iter().filter(|x| x.pri.is_none()) {
        println!("{}", todo);
    }

    Ok(())
}
