use colored::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub pri: Option<String>,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub done_date: Option<String>,
    pub due_date: Option<String>,
}

impl Todo {
    fn case_insensitive_match(haystack: &str, needle: &str) -> bool {
        haystack
            .to_ascii_lowercase()
            .contains(&needle.to_ascii_lowercase())
    }
    pub fn matches(&self, positives: &[String], negatives: &[String]) -> bool {
        let taskstr = self.task.clone() + &self.projects.join(" ") + &self.contexts.join(" ");
        let has_all_pos = positives
            .iter()
            .all(|y| Todo::case_insensitive_match(&taskstr, y));
        let has_no_neg = !negatives
            .iter()
            .any(|y| Todo::case_insensitive_match(&taskstr, y));
        has_all_pos && has_no_neg
    }

    pub fn append_text(&mut self, text: &str) {
        self.task.push(' ');
        self.task.push_str(text);
    }

    pub fn prepend_text(&mut self, text: &str) {
        self.task = format!("{} {}", text, self.task);
    }

    pub fn mark_done(&mut self) {
        self.done_date = Some(crate::utility::get_formatted_date());
    }

    pub fn mark_undone(&mut self) {
        self.done_date = None;
    }

    pub fn schedule(&mut self, date: &str) {
        self.due_date = Some(String::from(date));
    }

    pub fn unschedule(&mut self) {
        self.due_date = None;
    }

    pub fn schedule_today(&mut self) {
        self.due_date = Some(crate::utility::get_formatted_date());
    }

    pub fn format_for_save(&self) -> String {
        let mut to_output: Vec<String> = vec![];

        if let Some(p) = self.pri.as_ref() {
            to_output.push(format!("({})", p));
        };

        if let Some(done) = &self.done_date {
            to_output.push(format!("x {}", done))
        };

        to_output.push(self.task.to_string());

        if let Some(due) = &self.due_date {
            to_output.push(format!("due:{}", due))
        };

        let projects: String = self.projects.join(" ");
        if !projects.is_empty() {
            to_output.push(projects);
        }

        let contexts: String = self.contexts.join(" ");
        if !contexts.is_empty() {
            to_output.push(contexts);
        }

        to_output.join(" ")
    }

    pub fn donesummary_format(&self) -> String {
        let mut to_output: Vec<String> = vec![self.task.to_string()];

        let projects: String = self.projects.join(" ");
        if !projects.is_empty() {
            to_output.push(projects);
        }

        let contexts: String = self.contexts.join(" ");
        if !contexts.is_empty() {
            to_output.push(contexts);
        }

        to_output.join(" ")
    }
}

// Implement .parse() for Todo
impl std::str::FromStr for Todo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let (parts, contexts): (Vec<&str>, Vec<&str>) =
            parts.iter().partition(|p| !p.starts_with('@'));
        let (parts, projects): (Vec<&str>, Vec<&str>) =
            parts.iter().partition(|p| !p.starts_with('+'));
        let (parts, done_date) = if parts[0] == "x" {
            (parts[2..].to_vec(), Some(parts[1].to_string()))
        } else {
            (parts, None)
        };
        let (parts, priority) = if parts[0].starts_with('(') && parts[0].ends_with(')') {
            (parts[1..].to_vec(), Some(parts[0][1..2].to_string()))
        } else {
            (parts, None)
        };

        let (task_parts, due_date_maybe): (Vec<&str>, Vec<&str>) =
            parts.iter().partition(|x| !x.starts_with("due:"));
        let task = task_parts.join(" ");
        let due_date = due_date_maybe
            .get(0)
            .map(|x| x.split(':').nth(1).unwrap().to_string());

        Ok(Todo {
            idx: 0,
            task,
            pri: priority,
            projects: projects.iter().map(|x| x.to_string()).collect(),
            contexts: contexts.iter().map(|x| x.to_string()).collect(),
            done_date,
            due_date,
        })
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut to_colour: Vec<String> = vec![format!("{:4}.", self.idx)];

        if let Some(p) = self.pri.as_ref() {
            to_colour.push(format!("({})", p));
        };

        if let Some(done) = &self.done_date {
            to_colour.push(format!("x {}", done))
        };

        to_colour.push(self.task.to_string());

        if let Some(due) = &self.due_date {
            to_colour.push(format!("due:{}", due))
        };

        let to_colour = to_colour.join(" ");

        let pre = match self.pri.as_deref() {
            Some("A") => to_colour.yellow(),
            Some("B") => to_colour.green(),
            Some("C") => to_colour.blue(),
            _ => to_colour.white(),
        };

        let mut to_output = vec![pre.to_string()];

        let projects: String = self.projects.join(" ");
        if !projects.is_empty() {
            to_output.push(projects.red().to_string());
        }

        let contexts: String = self.contexts.join(" ");
        if !contexts.is_empty() {
            to_output.push(contexts.red().to_string());
        }

        write!(f, "{}", to_output.join(" "))
    }
}
