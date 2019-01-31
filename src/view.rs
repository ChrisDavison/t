#![allow(unused_variables)]
use std::collections::hash_map::HashMap;

use regex::Regex;

use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

lazy_static! {
    static ref re_ctx: Regex = Regex::new(r"@(.+?)\b").expect("Couldn't compile context regex");
    static ref re_proj: Regex = Regex::new(r"\+(.+?)\b").expect("Couldn't compile project regex");
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

pub fn contexts(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(false)?;;
    println!("{} tasks", todos.len());
    for (project, count) in count_regex_matches(&re_ctx)? {
        println!("{:5} {}", count, project);
    }
    Ok(())
}

pub fn projects(args: &[String]) -> Result<()> {
    let todos = utility::get_todos(false)?;;
    println!("{} tasks", todos.len());
    for (project, count) in count_regex_matches(&re_proj)? {
        println!("{:5} {}", count, project);
    }
    Ok(())
}

pub fn contextless(args: &[String]) -> Result<()> {
    for (i, line) in utility::get_todos(true)? {
        if !re_ctx.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}

pub fn projectless(args: &[String]) -> Result<()> {
    for (i, line) in utility::get_todos(true)? {
        if !re_proj.is_match(&line) {
            println!("{:5}\t{}", i, &line[2..]);
        }
    }
    Ok(())
}
