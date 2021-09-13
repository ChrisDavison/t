use super::{todo, utility};

use chrono::{Date, Duration, NaiveDate, Utc};
use std::collections::HashMap;

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
    let mut todos = utility::filter_todos(todos, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    Ok(())
}

pub fn done(dones: &[todo::Todo], filters: &[String]) -> Result<()> {
    let mut todos = utility::filter_todos(dones, filters);
    todos.sort_by(|a, b| a.pri.cmp(&b.pri));
    for todo in todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    for todo in todos.iter().filter(|x| x.pri.is_none()) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn done_summary(dones: &[todo::Todo], n_days: usize, filters: &[String]) -> Result<()> {
    let today = Utc::now().date();
    let mut last_week = HashMap::new();

    for done in utility::filter_todos(dones, filters) {
        if let Some(d) = utility::parse_date(done.done_date.as_ref()) {
            let task_date = Date::from_utc(d, *today.offset());
            let delta = (today - task_date).num_days();
            if delta < n_days as i64 {
                let entry = last_week.entry(delta).or_insert_with(Vec::new);
                entry.push(done.clone());
            }
        }
    }

    for i in (0..=6).rev() {
        match last_week.get(&i) {
            Some(dones) => {
                let that_day = today - Duration::days(i);
                println!("... {} ..........", that_day.format("%Y-%m-%d"));
                for d in dones {
                    println!("    {}", d.donesummary_format());
                }
                println!();
            }
            None => continue,
        }
    }

    Ok(())
}

pub fn due(todos: &[todo::Todo], n_days: usize, filters: &[String]) -> Result<()> {
    let todos = utility::filter_todos(todos, filters);

    let mut datediffed_todos: Vec<(i64, todo::Todo)> = todos
        .iter()
        .filter(|x| x.due_date.is_some())
        .map(|x| (days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(datediff1));
    for (days_overdue, t) in datediffed_todos {
        let pre = if days_overdue > 0 {
            format!("OVERDUE {} days:", days_overdue)
        } else if days_overdue == 0 {
            format!("TODAY:")
        } else {
            let days_till = days_overdue.abs() as usize;
            if days_till > n_days {
                continue;
            }
            format!("IN {} days:", days_till)
        };
        println!("{:15} {}", pre, t);
    }
    Ok(())
}

pub fn no_date(todos: &[todo::Todo], filters: &[String]) -> Result<()> {
    let todos = utility::filter_todos(todos, filters);
    let undated_todos: Vec<_> = todos.iter().filter(|x| x.due_date.is_none()).collect();
    for &todo in undated_todos.iter().filter(|x| x.pri.is_some()) {
        println!("{}", todo);
    }
    for todo in undated_todos.iter().filter(|x| x.pri.is_none()) {
        println!("{}", todo);
    }

    Ok(())
}
