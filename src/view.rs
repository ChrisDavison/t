use std::collections::hash_map::HashMap;

use chrono::{Date, NaiveDate, Utc};
use regex::{Captures, Regex};

use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;
type GroupedTasks = HashMap<String, Vec<(usize, String)>>;

lazy_static! {
    static ref re_ctx: Regex = Regex::new(r"@(.+?)\b").expect("Couldn't compile context regex");
    static ref re_proj: Regex = Regex::new(r"\+(.+?)\b").expect("Couldn't compile project regex");
    static ref re_due: Regex =
        Regex::new(r"due:(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    static ref re_pri: Regex = Regex::new(r"^- ! (.*)").expect("Couldn't compile priority regex");
}

fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(&needle.to_ascii_lowercase())
}

pub fn list(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(true)?;
    let query = match args.get(0) {
        Some(q) => q,
        None => "",
    };
    let filtered = todos
        .iter()
        .filter(|(_, x)| case_insensitive_match(x, query));
    for (i, line) in filtered {
        println!("{:5}\t{}", i, &line[2..]);
    }
    Ok(())
}

pub fn list_priorities() -> Result<()> {
    for (_, lines) in group_by_regex(&re_pri)? {
        for (i, line) in lines {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn hide(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(true)?;
    let query = match args.get(0) {
        Some(q) => q,
        None => "",
    };
    let filtered = todos
        .iter()
        .filter(|(_, x)| !case_insensitive_match(x, query));
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

pub fn group_by_regex(r: &Regex) -> Result<GroupedTasks> {
    let mut map: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    for (i, line) in utility::get_todos(true)? {
        for cap in r.captures_iter(&line) {
            let c = cap[0].to_owned();
            if let Some(x) = map.get_mut(&c) {
                (*x).push((i, line.clone()))
            } else {
                map.insert(c, vec![(i, line.clone())]);
            }
        }
    }
    Ok(map)
}

pub fn contexts() -> Result<()> {
    println!("{} tasks", utility::get_todos(false)?.len());
    for (header, lines) in group_by_regex(&re_ctx)? {
        println!("{:5}\t{}", lines.len(), header);
    }
    Ok(())
}

pub fn projects() -> Result<()> {
    println!("{} tasks", utility::get_todos(false)?.len());
    for (header, lines) in group_by_regex(&re_proj)? {
        println!("{:5}\t{}", lines.len(), header);
    }
    Ok(())
}

pub fn context_view() -> Result<()> {
    let grouped = group_by_regex(&re_ctx)?;
    let mut keys: Vec<String> = grouped.keys().map(|x| x.to_owned()).collect();
    keys.sort();
    for header in keys {
        println!("{}", header);
        for (i, line) in &grouped[&header] {
            println!("\t{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn project_view() -> Result<()> {
    let grouped = group_by_regex(&re_proj)?;
    let mut keys: Vec<String> = grouped.keys().map(|x| x.to_owned()).collect();
    keys.sort();
    for header in keys {
        println!("{}", header);
        for (i, line) in &grouped[&header] {
            println!("\t{:5}\t{}", i, &line[2..]);
        }
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

pub fn due() -> Result<()> {
    let mut map: HashMap<i64, Vec<(String, usize, String)>> = HashMap::new();
    for (i, line) in utility::get_todos(false)? {
        for cap in re_due.captures_iter(&line) {
            let diff = get_datediff(&cap)?;
            if let Some(x) = map.get_mut(&diff) {
                (*x).push((cap[0].to_string(), i, line.clone()));
            } else {
                map.insert(diff, vec![(cap[0].to_string(), i, line.clone())]);
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

pub fn no_date() -> Result<()> {
    for (i, line) in utility::get_todos(true)? {
        if !re_due.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn mit() -> Result<()> {
    let mut map: HashMap<i64, Vec<(String, usize, String)>> = HashMap::new();
    for (i, line) in utility::get_todos(false)? {
        for cap in re_due.captures_iter(&line) {
            let diff = get_datediff(&cap)?;
            if diff < 0 {
                continue;
            }
            if let Some(x) = map.get_mut(&diff) {
                (*x).push((cap[0].to_string(), i, line.clone()));
            } else {
                map.insert(diff, vec![(cap[0].to_string(), i, line.clone())]);
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
