use std::fmt;

use regex::Regex;

#[derive(Clone)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub priority: bool,
    pub date: String,
    pub done: String,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = if self.priority { " ! " } else { "" };
        let d = if self.date != "" {
            format!("{} ", self.date).to_string()
        } else {
            String::new()
        };
        let dd = if self.done != "" {
            format!("{} ", self.done).to_string()
        } else {
            String::new()
        };
        write!(f, "{:3}{:3}. {}{:11}{}", p, self.idx, dd, d, self.task)
    }
}

pub fn parse_todo(idx: usize, line: &str) -> Todo {
    // let line = &line[2..];
    let (task, priority) = if line.starts_with("! ") {
        (&line[2..], true)
    } else {
        (line, false)
    };
    let re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    let (task, date) = if re_date.is_match(task) {
        (&task[11..], &task[..10])
    } else {
        (task, "")
    };
    Todo {
        idx,
        task: task.to_string(),
        priority,
        date: date.to_string(),
        done: "".to_string(),
    }
}

pub fn parse_done(idx: usize, line: &str) -> Todo {
    let line = &line[2..];
    let (done, task, priority) = if line.starts_with("! ") {
        (&line[2..12], &line[13..], true)
    } else {
        (&line[0..10], &line[11..], false)
    };
    let re_date: Regex =
        Regex::new(r"(\d{4})-(\d{2})-(\d{2})").expect("Couldn't compile date regex");
    let (task, date) = if re_date.is_match(task) {
        (&task[11..], &task[..10])
    } else {
        (task, "")
    };
    Todo {
        idx,
        task: task.to_string(),
        priority,
        date: date.to_string(),
        done: done.to_string(),
    }
}

pub fn printable(t: &Todo) -> String {
    let p = if t.priority { "! " } else { "" };
    let dt = if t.date != "" {
        format!("{} ", t.date).to_string()
    } else {
        "".to_string()
    };
    let done = if t.done != "" {
        format!("{} ", t.done).to_string()
    } else {
        String::new()
    };
    let msg = format!("- {}{}{}{}\n", done, p, dt, t.task);
    let re_spc: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
    re_spc.replace(&msg, " ").to_string()
}
