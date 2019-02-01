use std::collections::hash_map::HashMap;

use chrono::{Date, NaiveDate, Utc};
use regex::{Captures, Regex};

use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

lazy_static! {
    static ref re_ctx: Regex = Regex::new(r"@(.+?)\b").expect("Couldn't compile context regex");
    static ref re_proj: Regex = Regex::new(r"\+(.+?)\b").expect("Couldn't compile project regex");
    static ref re_due: Regex =
        Regex::new(r"due:(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
}

pub fn list(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(true)?;
    let query = match args.get(0) {
        Some(q) => q,
        None => "",
    };
    let filtered = todos.iter().filter(|(_, x)| x.contains(query));
    for (i, line) in filtered {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

pub fn list_priorities() -> Result<()> {
    let todos = utility::get_todos(true)?;
    let filtered = todos.iter().filter(|(_, x)| x.starts_with("- ! "));
    for (i, line) in filtered {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

pub fn hide(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(true)?;
    let query = match args.get(0) {
        Some(q) => q,
        None => "",
    };
    let filtered = todos.iter().filter(|(_, x)| !x.contains(query));
    for (i, line) in filtered {
        println!("{:5}\t{}", i, line);
    }
    Ok(())
}

pub fn done() -> Result<()> {
    let todos = utility::get_done()?;
    for (i, line) in todos {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

fn count_regex_matches(r: &Regex) -> Result<HashMap<String, usize>> {
    let mut map = HashMap::new();
    for (_, line) in utility::get_todos(false)? {
        for cap in r.captures_iter(&line) {
            let c = cap[0].to_owned();
            if let Some(x) = map.get(&c) {
                map.insert(c, x + 1);
            } else {
                map.insert(c, 1);
            }
        }
    }
    Ok(map)
}

pub fn contexts() -> Result<()> {
    let todos = utility::get_todos(false)?;;
    println!("{} tasks", todos.len());
    for (project, count) in count_regex_matches(&re_ctx)? {
        println!("{:5} {}", count, project);
    }
    Ok(())
}

pub fn projects() -> Result<()> {
    let todos = utility::get_todos(false)?;;
    println!("{} tasks", todos.len());
    for (project, count) in count_regex_matches(&re_proj)? {
        println!("{:5} {}", count, project);
    }
    Ok(())
}

pub fn contextless() -> Result<()> {
    for (i, line) in utility::get_todos(true)? {
        if !re_ctx.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn projectless() -> Result<()> {
    for (i, line) in utility::get_todos(true)? {
        if !re_proj.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

fn get_datediff(capture: &Captures) -> Result<i64> {
    let now: Date<Utc> = Utc::now().date();
    let y = capture[1].parse()?;
    let m = capture[2].parse()?;
    let d = capture[3].parse()?;
    let task_date = Date::from_utc(NaiveDate::from_ymd(y, m, d), *now.offset());
    Ok((now - task_date).num_days())
}

pub fn due(args: &[String]) -> Result<()> {
    let mut map: HashMap<i64, Vec<(usize, String)>> = HashMap::new();
    for (i, line) in utility::get_todos(false)? {
        for cap in re_due.captures_iter(&line) {
            let diff = get_datediff(&cap)?;
            if let Some(x) = map.get_mut(&diff) {
                (*x).push((i, line.clone()));
            } else {
                map.insert(diff, vec![(i, line.clone())]);
            }
        }
    }
    let future = if args.is_empty() {
        -999
    } else {
        args[0].parse()?
    };
    let mut keys: Vec<i64> = map.keys().cloned().collect();
    keys.sort();
    keys.reverse();
    for days in keys {
        if days > 0 {
            println!("{} days overdue", days);
        } else if days == 0 {
            println!("DUE TODAY");
        } else if days < future {
            println!("Due in {} days", -days)
        } else {
            continue;
        }
        for (i, line) in &map[&days] {
            println!("\t{}\t{}", i, &line[2..]);
        }
        println!();
    }
    Ok(())
}
