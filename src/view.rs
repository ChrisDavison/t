use super::{
    todo::Todo,
    utility::{self, todo_filter},
};

use chrono::Duration;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn list<'a>(todos: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    for todo in utility::sort_by_priority(todo_filter(todos, filters)) {
        println!("{}", todo);
    }

    Ok(())
}

pub fn list_priority<'a>(todos: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    let sorted = utility::sort_by_priority(
        todo_filter(todos, filters).filter(|t| !matches!(t.pri, crate::todo::TodoPriority::None)),
    );
    for todo in sorted {
        println!("{}", todo);
    }
    Ok(())
}

pub fn done<'a>(dones: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    for done in &utility::sort_by_priority(todo_filter(dones, filters)) {
        println!("{}", done);
    }

    Ok(())
}

pub fn done_summary<'a>(dones: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    let today = utility::date_today();
    let mut last_week = HashMap::new();

    let n_days = std::env::var("T_DONESUMMARY_DAYS")
        .unwrap_or_else(|_| "7".to_string())
        .parse()?;

    for done in utility::todo_filter(dones, filters) {
        let delta = done.days_since_done()?;
        if delta < n_days {
            let entry = last_week.entry(delta).or_insert_with(Vec::new);
            entry.push(done.clone());
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

pub fn due<'a>(
    todos: impl Iterator<Item = &'a Todo>,
    n_days: usize,
    filters: &[String],
) -> Result<()> {
    let mut datediffed_todos = Vec::new();
    for t in utility::todo_filter(todos, filters) {
        if t.due_date.is_some() {
            datediffed_todos.push((t.days_overdue()?, t.to_owned()));
        }
    }
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(datediff1));
    for (days_overdue, t) in datediffed_todos {
        let days_in_future = days_overdue.abs() as usize;
        if days_overdue < 0 && days_in_future > n_days {
            // Too far in future
            continue;
        }
        let pre = match days_overdue {
            0 => "TODAY:".to_string(),
            1.. => format!("OVERDUE {} days:", days_overdue),
            _ => format!("IN {} days:", days_in_future),
        };
        println!("{:15} {}", pre, t);
    }
    Ok(())
}

pub fn no_date<'a>(todos: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    let undated_todos = todo_filter(todos, filters).filter(|todo| todo.due_date.is_none());
    for todo in utility::sort_by_priority(undated_todos) {
        println!("{}", todo);
    }
    Ok(())
}

pub fn projects<'a>(todos: impl Iterator<Item = &'a Todo>) -> Result<()> {
    let mut projects = HashMap::new();
    for t in todos {
        for project in &t.projects {
            let entry = projects.entry(project).or_insert(0);
            *entry += 1;
        }
    }
    for (p, n) in projects {
        println!("{} {}", p, n);
    }
    Ok(())
}

pub fn contexts<'a>(todos: impl Iterator<Item = &'a Todo>) -> Result<()> {
    let mut contexts = HashMap::new();
    for t in todos {
        for context in &t.contexts {
            let entry = contexts.entry(context).or_insert(0);
            *entry += 1;
        }
    }
    for (c, n) in contexts {
        println!("{} {}", c, n);
    }
    Ok(())
}
