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

pub fn list(todos: &[(usize, todo::Todo)], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    for (idx, todo) in todos {
        println!("{:4}. {}", idx, todo);
    }
    Ok(())
}

pub fn done(todos: &[(usize, todo::Todo)], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for (idx, todo) in todos {
        println!("{:4}. {}", idx, todo);
    }
    Ok(())
}

pub fn due(todos: &[(usize, todo::Todo)], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);

    let mut datediffed_todos: Vec<(usize, i64, todo::Todo)> = todos
        .iter()
        .filter(|(_idx, x)| x.kws.contains_key("due"))
        .map(|(idx, x)| (*idx, days_overdue(x), x.to_owned()))
        .collect();
    datediffed_todos.sort_by(|(_, datediff1, _), (_, datediff2, _)| datediff2.cmp(&datediff1));
    for (idx, _diff, t) in datediffed_todos {
        println!("{:4}. {}", idx, t);
    }
    Ok(())
}

pub fn no_date(todos: &[(usize, todo::Todo)], args: &[String]) -> Result<()> {
    let (todos, _args) = utility::filter_todos(&todos, &args);
    let undated_todos = todos.iter().filter(|(_idx, x)| !x.kws.contains_key("due"));
    for (idx, t) in undated_todos {
        println!("{:4}. {}", idx, t);
    }
    Ok(())
}
