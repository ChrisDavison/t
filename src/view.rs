use super::utility;

use std::collections::hash_map::HashMap;

use chrono::{Date, NaiveDate, Utc};
use regex::{Captures, Regex};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
type GroupedTasks = HashMap<String, Vec<(usize, String)>>;

lazy_static! {
    static ref re_proj: Regex = Regex::new(r"\+(.+?)\b").expect("Couldn't compile project regex");
    pub static ref re_due: Regex =
        Regex::new(r"due:(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    static ref re_pri: Regex = Regex::new(r"^- ! (.*)").expect("Couldn't compile priority regex");
}

pub fn list(todos: &[(usize, String)], args: &[String]) -> Result<()> {
    let (todos, args) = utility::filter_todos(&todos, &args);
    let query = match args.get(0) {
        Some(q) => q,
        None => "",
    };
    let filtered = todos
        .iter()
        .filter(|(_, x)| utility::case_insensitive_match(x, query));
    for (i, line) in filtered {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

pub fn list_priorities(todos: &[(usize, String)], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for (_, lines) in group_by_regex(&todos, &re_pri)? {
        for (i, line) in lines {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn done(todos: &[(usize, String)], args: &[String]) -> Result<()> {
    let (todos, _) = utility::filter_todos(&todos, &args);
    for (i, line) in todos {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

pub fn group_by_regex(todos: &[(usize, String)], r: &Regex) -> Result<GroupedTasks> {
    let mut map: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for (i, line) in todos {
        for cap in r.captures_iter(&line) {
            let c = cap[0].to_owned();
            if let Some(x) = map.get_mut(&c) {
                (*x).push((*i, line.clone()))
            } else {
                map.insert(c, vec![(*i, line.clone())]);
            }
        }
    }
    Ok(map)
}

pub fn projects(todos: &[(usize, String)]) -> Result<()> {
    println!("{} tasks", todos.len());
    let grouped = group_by_regex(todos, &re_proj)?;
    let mut keys: Vec<String> = grouped.keys().map(|x| x.to_owned()).collect();
    keys.sort();
    for header in keys {
        if let Some(lines) = grouped.get(&header) {
            println!("{:5}\t{}", lines.len(), header);
        }
    }
    Ok(())
}

pub fn project_view(todos: &[(usize, String)]) -> Result<()> {
    let grouped = group_by_regex(todos, &re_proj)?;
    let mut keys: Vec<String> = grouped.keys().map(|x| x.to_owned()).collect();
    keys.sort();
    let max = keys
        .iter()
        .map(|x| x.len())
        .max()
        .expect("Couldn't get longest key");
    for header in keys {
        for (i, line) in &grouped[&header] {
            println!("{:width$}\t{:5}\t{}", header, i, &line[2..], width = max);
        }
    }
    Ok(())
}

pub fn projectless(todos: &[(usize, String)]) -> Result<()> {
    for (i, line) in todos {
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

pub fn due(todos: &[(usize, String)]) -> Result<()> {
    let mut map: HashMap<i64, Vec<(String, usize, String)>> = HashMap::new();
    for (i, line) in todos {
        for cap in re_due.captures_iter(&line) {
            let diff = get_datediff(&cap)?;
            if let Some(x) = map.get_mut(&diff) {
                (*x).push((cap[0].to_string(), *i, line.clone()));
            } else {
                map.insert(diff, vec![(cap[0].to_string(), *i, line.clone())]);
            }
        }
    }

    let mut keys: Vec<i64> = map.keys().cloned().collect();
    keys.sort();
    keys.reverse();
    for days in keys {
        for (date, i, line) in &map[&days] {
            println!("{:10}\t{:5}\t{}", &date[4..], i, &line[2..]);
        }
    }
    Ok(())
}

pub fn no_date(todos: &[(usize, String)]) -> Result<()> {
    for (i, line) in todos {
        if !re_due.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn mit(todos: &[(usize, String)]) -> Result<()> {
    let mut map: HashMap<i64, Vec<(String, usize, String)>> = HashMap::new();
    for (i, line) in todos {
        for cap in re_due.captures_iter(&line) {
            let diff = get_datediff(&cap)?;
            if diff < 0 {
                continue;
            }
            if let Some(x) = map.get_mut(&diff) {
                (*x).push((cap[0].to_string(), *i, line.clone()));
            } else {
                map.insert(diff, vec![(cap[0].to_string(), *i, line.clone())]);
            }
        }
    }
    let mut keys: Vec<i64> = map.keys().cloned().collect();
    keys.sort();
    keys.reverse();
    for days in keys {
        for (date, i, line) in &map[&days] {
            if !line.starts_with("- ! ") {
                continue;
            }
            println!("{}\t{:-5}\t{}", &date[4..], i, &line[4..]);
        }
    }
    Ok(())
}
