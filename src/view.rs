use super::{
    todo::Todo,
    utility::{self, todo_filter},
};

use chrono::Duration;
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn print_todos<'a>(todos: impl Iterator<Item = &'a Todo>) {
    println!("{}", todos.map(|x| x.to_string()).collect::<Vec<String>>().join("\n")); 
}

pub fn list<'a>(todos: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    print_todos(utility::sort_by_priority(todo_filter(todos, filters)).iter());

    Ok(())
}

pub fn list_priority<'a>(todos: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    let sorted = utility::sort_by_priority(
        todo_filter(todos, filters).filter(|t| !matches!(t.pri, crate::todo::TodoPriority::None)),
    );
    print_todos(sorted.iter());
    Ok(())
}

pub fn done<'a>(dones: impl Iterator<Item = &'a Todo>, filters: &[String]) -> Result<()> {
    print_todos(utility::sort_by_priority(todo_filter(dones, filters)).iter());

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
        let days_in_future: usize = days_overdue.unsigned_abs() as usize;
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
    let mut n_no_project = 0;
    for t in todos {
        if t.projects.is_empty() {
            n_no_project += 1;
        }
        for project in &t.projects {
            let entry = projects.entry(project).or_insert(0);
            *entry += 1;
        }
    }
    for (p, n) in projects {
        println!("{} {}", p, n);
    }
    println!("NO PROJECT {}", n_no_project);
    Ok(())
}

pub fn contexts<'a>(todos: impl Iterator<Item = &'a Todo>) -> Result<()> {
    let mut contexts = HashMap::new();
    let mut n_no_context = 0;
    for t in todos {
        if t.contexts.is_empty() {
            n_no_context += 1;
            continue;
        }
        for context in &t.contexts {
            let entry = contexts.entry(context).or_insert(0);
            *entry += 1;
        }
    }
    for (c, n) in contexts {
        println!("{} {}", c, n);
    }
    println!("NO CONTEXT {}", n_no_context);
    Ok(())
}

pub fn grouped_by_project<'a>(
    todos: impl Iterator<Item = &'a Todo>,
    filters: &[String],
) -> Result<()> {
    let mut projects = HashMap::new();
    let mut no_project = Vec::new();
    let sorted_and_filtered = utility::sort_by_priority(todo_filter(todos, filters));
    for t in &sorted_and_filtered {
        if t.projects.is_empty() {
            no_project.push(t);
        } else {
            for project in &t.projects {
                let entry = projects.entry(project).or_insert(vec![]);
                (*entry).push(t);
            }
        }
    }
    for (p, todos_for_project) in projects {
        println!("{}", p);
        for todo in todos_for_project {
            println!("{}", todo);
        }
        println!();
    }
    println!("NO PROJECT");
    for todo in no_project {
        println!("{}", todo);
    }
    println!();
    Ok(())
}

pub fn grouped_by_context<'a>(
    todos: impl Iterator<Item = &'a Todo>,
    filters: &[String],
) -> Result<()> {
    let mut contexts = HashMap::new();
    let mut no_context = Vec::new();
    let sorted_and_filtered = utility::sort_by_priority(todo_filter(todos, filters));
    for t in &sorted_and_filtered {
        if t.contexts.is_empty() {
            no_context.push(t);
        } else {
            for context in &t.contexts {
                let entry = contexts.entry(context).or_insert(vec![]);
                (*entry).push(t);
            }
        }
    }
    for (c, todos_for_context) in contexts {
        println!("{}", c);
        for todo in todos_for_context {
            println!("{}", todo);
        }
        println!();
    }
    println!("NO CONTEXT");
    for todo in no_context {
        println!("{}", todo);
    }
    println!();
    Ok(())
}
