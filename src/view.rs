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

pub fn done_summary<'a>(
    dones: impl Iterator<Item = &'a Todo>,
    filters: &[String],
    n_days: i64,
) -> Result<()> {
    let today = utility::date_today();
    let mut last_week = HashMap::new();

    for done in utility::todo_filter(dones, filters) {
        let delta = done.days_since_done()?;
        if delta < n_days {
            let entry = last_week.entry(delta).or_insert_with(Vec::new);
            entry.push(done.clone());
        }
    }

    for i in (0..=n_days).rev() {
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
    let mut prev = None;
    for (days_overdue, t) in datediffed_todos {
        let days_in_future = days_overdue.abs() as usize;
        if days_overdue < 0 && days_in_future > n_days {
            // Too far in future
            continue;
        }
        let header = match days_overdue {
            0 => "Today".to_string(),
            1.. => format!("Overdue {} days", days_overdue),
            _ => format!("In {} days", days_in_future),
        };
        let header = format!("..... {} {}", header, ".".repeat(73 - header.len()));
        match prev {
            Some(p) => {
                if days_overdue != p {
                    println!("\n{}\n", header);
                }
            }
            None => println!("{}\n", header),
        }
        println!("{}", t);
        prev = Some(days_overdue);
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

pub fn grouped_by_project<'a>(
    todos: impl Iterator<Item = &'a Todo>,
    filters: &[String],
) -> Result<()> {
    let mut projects = HashMap::new();
    for t in
        todo_filter(todos, filters).filter(|t| !matches!(t.pri, crate::todo::TodoPriority::None))
    {
        for project in &t.projects {
            let entry = projects.entry(project).or_insert(vec![]);
            (*entry).push(t);
        }
    }
    for (p, todos_for_project) in projects {
        println!("{}", p);
        for todo in todos_for_project {
            println!("{}", todo);
        }
        println!();
    }
    Ok(())
}
