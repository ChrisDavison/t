use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_CONTEXT: Regex =
        Regex::new(r"@([a-zA-Z0-9\-]+)").expect("Couldn't compile context regex");
    static ref RE_TAG: Regex =
        Regex::new(r"\+([a-zA-Z0-9\-]+)").expect("Couldn't compile tag regex");
    static ref RE_KEYWORD: Regex =
        Regex::new(r"([a-zA-Z]+):([a-zA-Z0-9\-]+)").expect("Couldn't compile keyword regex");
    static ref RE_SPC: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub task: String,
    pub kws: HashMap<String, String>,
    pub projects: Vec<String>,
    pub tags: Vec<String>,
}

// Implement .parse() for Todo
impl std::str::FromStr for Todo {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut task_new = s.to_string();
        let mut projects = Vec::new();
        for cap in RE_CONTEXT.captures_iter(s) {
            projects.push(cap[1].to_string());
            task_new = task_new.replace(&cap[0], "").to_string();
        }

        let mut tags = Vec::new();
        for cap in RE_TAG.captures_iter(&task_new.clone()) {
            tags.push(cap[1].to_string());
            task_new = task_new.replace(&cap[0], "").to_string();
        }

        let mut kws = HashMap::new();
        let mut task_new = task_new.clone();
        for cap in RE_KEYWORD.captures_iter(&task_new.clone()) {
            kws.insert(cap[1].to_string(), cap[2].to_string());
            task_new = task_new.replace(&cap[0], "").to_string();
        }

        Ok(Todo {
            task: task_new.to_string(),
            kws: kws,
            projects: projects,
            tags: tags,
        })
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = self.kws.get("due").unwrap_or(&String::new()).to_string();
        let dd = self.kws.get("done").unwrap_or(&String::new()).to_string();
        let projects = self
            .projects
            .iter()
            .map(|x| String::from("@") + x)
            .collect::<Vec<String>>()
            .join(" ");
        let tags = self
            .tags
            .iter()
            .map(|x| String::from("@") + x)
            .collect::<Vec<String>>()
            .join(" ");
        let keywords = self
            .kws
            .iter()
            .filter(|(k, _)| k != &"due" && k != &"done")
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join(" ");
        let mut msg = String::new();
        let mut prev = self.task.chars().nth(0).unwrap();
        for char in format!("{}{}{}{}", self.task, projects, tags, keywords).chars() {
            if prev == ' ' && char == ' ' {
                continue;
            }
            msg.push(char);
            prev = char;
        }
        write!(f, "{}{:11}{}", dd, d, msg)
    }
}

pub fn printable(t: &Todo) -> String {
    let dt = t.kws.get("due").unwrap_or(&String::new()).to_string();
    let done = t.kws.get("done").unwrap_or(&String::new()).to_string();
    let msg = format!("- {}{}{}\n", done, dt, t.task);
    RE_SPC.replace(&msg, " ").to_string()
}
