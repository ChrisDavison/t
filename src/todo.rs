use std::fmt;

use regex::Regex;

#[derive(Clone)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub date: String,
    pub done: String,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        write!(f, "{:3}. {}{:11}{}", self.idx, dd, d, self.task)
    }
}

pub fn parse_todo(idx: usize, task: &str) -> Todo {
    // let task = &task[2..];
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
        date: date.to_string(),
        done: "".to_string(),
    }
}

pub fn parse_done(idx: usize, line: &str) -> Todo {
    let line = &line[2..];
    let (done, task) = (&line[0..10], &line[11..]);
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
        date: date.to_string(),
        done: done.to_string(),
    }
}

pub fn printable(t: &Todo) -> String {
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
    let msg = format!("- {}{}{}\n", done, dt, t.task);
    let re_spc: Regex = Regex::new(r"\s\s+").expect("Couldn't compile space regex");
    re_spc.replace(&msg, " ").to_string()
}
