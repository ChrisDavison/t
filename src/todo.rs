use colored::*;
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_CONTEXT: Regex =
        Regex::new(r"@([a-zA-Z0-9\-]+)").expect("Couldn't compile context regex");
    static ref RE_TAG: Regex =
        Regex::new(r"\+([a-zA-Z0-9\-]+)").expect("Couldn't compile tag regex");
    static ref RE_PRI: Regex =
        Regex::new(r"\(([a-zA-Z]+)\)").expect("Couldn't compile priority regex");
    static ref RE_KEYWORD: Regex =
        Regex::new(r"([a-zA-Z]+):([a-zA-Z0-9\-]+)").expect("Couldn't compile keyword regex");
    pub static ref RE_SPC: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
}

#[derive(Clone, Debug)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub pri: String,
    pub kws: HashMap<String, String>,
    pub projects: Vec<String>,
    pub tags: Vec<String>,
}

impl Todo {
    fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
        haystack
            .to_ascii_lowercase()
            .contains(&needle.to_ascii_lowercase())
    }
    pub fn matches(&self, positives: &[String], negatives: &[String]) -> bool {
        let taskstr = self.task.clone() + &self.projects.join(" ") + &self.tags.join(" ");
        let has_all_pos = positives
            .iter()
            .all(|y| Todo::case_insensitive_match(&taskstr, y));
        let has_no_neg = !negatives
            .iter()
            .any(|y| Todo::case_insensitive_match(&taskstr, y));
        has_all_pos && has_no_neg
    }
}

// Implement .parse() for Todo
impl std::str::FromStr for Todo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut task = s.to_string();
        let mut projects = Vec::new();
        let mut pri = String::new();
        for cap in RE_PRI.captures_iter(s) {
            pri = cap[1].to_string();
            // task = task.replace(&cap[0], "").to_string();
        }

        for cap in RE_CONTEXT.captures_iter(s) {
            projects.push(cap[1].to_string());
            task = task.replace(&cap[0], "").to_string();
        }

        let mut tags = Vec::new();
        for cap in RE_TAG.captures_iter(&task.clone()) {
            tags.push(cap[1].to_string());
            task = task.replace(&cap[0], "").to_string();
        }

        let mut kws = HashMap::new();
        for cap in RE_KEYWORD.captures_iter(&task.clone()) {
            kws.insert(cap[1].to_string(), cap[2].to_string());
            task = task.replace(&cap[0], "").to_string();
        }

        Ok(Todo {
            idx: 0,
            task,
            pri,
            kws,
            projects,
            tags,
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
            .map(|x| String::from("+") + x)
            .collect::<Vec<String>>()
            .join(" ");
        let keywords = self
            .kws
            .iter()
            .filter(|(k, _)| k != &"due" && k != &"done")
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<String>>()
            .join(" ");
        let pre = format!("{:4}. {}{:11}{}", self.idx, dd, d, self.task);
        let pre = match self.pri.as_ref() {
            "A" => pre.yellow(),
            "B" => pre.green(),
            "C" => pre.blue(),
            _ => pre.white(),
        };
        let post = format!("{} {} {}", projects.red(), tags.red(), keywords);
        let post = RE_SPC.replace(&post, " ");
        write!(f, "{} {}", pre, post)
    }
}
