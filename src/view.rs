use super::{
    todo::Todo,
    utility::{self, todo_filter},
};

use chrono::{Date, Duration, NaiveDate, Utc};
use std::collections::HashMap;

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn days_overdue(t: &Todo) -> i64 {
    let now: Date<Utc> = utility::date_today();
    let naive = NaiveDate::parse_from_str(
        t.due_date.as_ref().unwrap_or(&String::from("")).as_ref(),
        "%Y-%m-%d",
    )
    .expect("Couldn't parse date");
    let task_date = Date::from_utc(naive, *now.offset());
    (now - task_date).num_days()
}

pub fn list(todos: &[Todo], filters: &[String]) -> Result<()> {
    let mut todos: Vec<Todo> = todo_filter(todos, filters).cloned().collect();
    todos.sort_by(|a, b| match (a.pri.as_ref(), b.pri.as_ref()) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    });
    for todo in todos {
        println!("{}", todo);
    }

    Ok(())
}

pub fn list_priority(todos: &[Todo], filters: &[String]) -> Result<()> {
    let mut todos = todos.to_vec();
    todos.sort_by(|a, b| match (a.pri.as_ref(), b.pri.as_ref()) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    });
    for todo in todo_filter(&todos, filters).filter(|t| t.pri.is_some()) {
        println!("{}", todo);
    }
    Ok(())
}

pub fn done(dones: &[Todo], filters: &[String]) -> Result<()> {
    let mut dones = dones.to_vec();
    dones.sort_by(|a, b| match (a.pri.as_ref(), b.pri.as_ref()) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    });
    for done in todo_filter(&dones, filters) {
        println!("{}", done);
    }

    Ok(())
}

pub fn done_summary(dones: &[Todo], filters: &[String]) -> Result<()> {
    let today = utility::date_today();
    let mut last_week = HashMap::new();

    let n_days = std::env::var("T_DONESUMMARY_DAYS")
        .unwrap_or_else(|_| "7".to_string())
        .parse()?;

    for done in utility::filter_todos(dones, filters) {
        if let Some(d) = utility::parse_date(done.done_date.as_ref()) {
            let task_date = Date::from_utc(d, *today.offset());
            let delta = (today - task_date).num_days();
            if delta < n_days {
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

pub fn due(todos: &[Todo], n_days: usize, filters: &[String]) -> Result<()> {
    let todos = utility::filter_todos(todos, filters);

    let mut datediffed_todos: Vec<(i64, Todo)> = todos
        .iter()
        .filter(|x| x.due_date.is_some())
        .map(|x| (days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(datediff1, _), (datediff2, _)| datediff2.cmp(datediff1));
    for (days_overdue, t) in datediffed_todos {
        let days_in_future = days_overdue.abs() as usize;
        if days_in_future > n_days {
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

pub fn no_date(todos: &[Todo], filters: &[String]) -> Result<()> {
    let mut undated_todos: Vec<Todo> = todo_filter(todos, filters)
        .filter(|todo| todo.due_date.is_none())
        .cloned()
        .collect();
    undated_todos.sort_by(|a, b| match (a.pri.as_ref(), b.pri.as_ref()) {
        (Some(a), Some(b)) => a.cmp(b),
        (Some(_), None) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Greater,
    });

    for todo in undated_todos {
        println!("{}", todo);
    }

    Ok(())
}

pub fn projects(todos: &[Todo]) -> Result<()> {
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

pub fn contexts(todos: &[Todo]) -> Result<()> {
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
