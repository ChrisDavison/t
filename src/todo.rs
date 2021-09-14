use super::utility;
use colored::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn case_insensitive_match(haystack: impl ToString, needle: impl ToString) -> bool {
        haystack
            .to_string()
            .to_ascii_lowercase()
            .contains(&needle.to_string().to_ascii_lowercase())
    }

    pub fn matches(&self, positives: &[impl ToString], negatives: &[impl ToString]) -> bool {
        let taskstr = self.task.clone() + &self.projects.join(" ") + &self.contexts.join(" ");
        let has_all_pos = positives
            .iter()
            .all(|y| Todo::case_insensitive_match(&taskstr, y.to_string()));
        let has_no_neg = !negatives
            .iter()
            .any(|y| Todo::case_insensitive_match(&taskstr, y.to_string()));
        has_all_pos && has_no_neg
    }

    pub fn append_text(&mut self, text: &str) {
        self.task.push(' ');
        self.task.push_str(text);
        utility::notify("APPENDED", &self);
    }

    pub fn prepend_text(&mut self, text: &str) {
        self.task = format!("{} {}", text, self.task);
        utility::notify("PREPENDED", &self);
    }

    pub fn prioritise(&mut self, priority: Option<String>) {
        self.pri = priority;
        if self.pri.is_some() {
            utility::notify("PRIORITISED", &self);
        } else {
            utility::notify("DEPRIORITISED", self);
        }
    }

    pub fn mark_done(&mut self) {
        self.done_date = Some(utility::date_today().format("%Y-%m-%d").to_string());
        utility::notify("DONE", &self);
    }

    pub fn mark_undone(&mut self) {
        self.done_date = None;
        utility::notify("UNDONE", &self);
    }

    pub fn schedule(&mut self, date: &str) {
        self.due_date = Some(utility::parse_date_string_relative(
            utility::date_today(),
            date,
        ));
        utility::notify("SCHEDULED", &self);
    }

    pub fn unschedule(&mut self) {
        self.due_date = None;
        utility::notify("UNSCHEDULED", &self);
    }

    pub fn format_for_save(&self) -> String {
        let mut to_output: Vec<String> = vec![];

        if let Some(done) = &self.done_date {
            to_output.push(format!("x {}", done));
        };

        if let Some(p) = self.pri.as_ref() {
            if self.done_date.is_none() {
                to_output.push(format!("({})", p));
            }
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
        let mut done_date = None;
        let mut priority = None;
        let mut task_parts = Vec::new();
        let mut projects = Vec::new();
        let mut contexts = Vec::new();
        let mut due_date = None;

        let token_iter: Vec<&str> = s.split_whitespace().collect();
        let is_priority =
            |word: &str| word.starts_with('(') && word.ends_with(')') && word.len() == 3;
        let mut idx = 0;
        loop {
            let token = match token_iter.get(idx) {
                None => break,
                Some(token) => *token,
            };

            if idx == 0 && token == "x" {
                done_date = Some(token_iter[1].to_string());
                idx = 2;
                continue;
            } else if is_priority(token) {
                priority = Some(token[1..2].to_string());
            } else if let Some(date) = token.strip_prefix("due:") {
                due_date = Some(date.to_string());
            } else if token.starts_with('@') {
                contexts.push(token);
            } else if token.starts_with('+') {
                projects.push(token);
            } else {
                task_parts.push(token);
            }
            idx += 1;
        }
        Ok(Todo {
            idx: 0,
            task: task_parts.join(" "),
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
        let mut to_colour: Vec<String> = vec![format!("{:3}.", self.idx)];

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

        let colourer = match self.pri.as_deref() {
            Some("A") => |s: String| s.yellow().to_string(),
            Some("B") => |s: String| s.green().to_string(),
            Some("C") => |s: String| s.blue().to_string(),
            _ => |s: String| s.to_string(),
        };

        let mut to_output = vec![colourer(to_colour)];

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

#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod tests {
    use crate::{todo::Todo, utility::date_today};

    #[test]
    fn can_parse_task() {
        let input = "this is a test +p1 +p2 @c1";
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        let got: Todo = input.parse().unwrap();
        assert_eq!(t, got);
    }

    fn can_parse_done_task() {
        let input = "x 2021-01-01 this is a test +p1 +p2 @c1";
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: Some("A".to_string()),
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some("2021-01-01".to_string()),
            due_date: None,
        };
        let got: Todo = input.parse().unwrap();
        assert_eq!(t, got);
    }

    #[test]
    fn can_append_text_to_tast() {
        let mut t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.append_text("EXTRA");

        let expected = Todo {
            idx: 0,
            task: "this is a test EXTRA".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn can_prepend_text_to_task() {
        let mut t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.prepend_text("EXTRA");

        let expected = Todo {
            idx: 0,
            task: "EXTRA this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn can_schedule_a_task() {
        let mut t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.schedule("today");

        let expected = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some(date_today().format("%F").to_string()),
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn can_unschedule_a_task() {
        let mut t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        t.unschedule();

        let expected = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn can_match_a_task() {
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };

        assert!(t.matches(&["test"], &["blah"]));
        assert!(!t.matches(&["test"], &["+p1"]));
        assert!(t.matches(&["test"], &["+badproj"]));
    }

    #[test]
    fn can_mark_task_as_done() {
        let mut input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some(date_today().format("%F").to_string()),
        };
        input.mark_done();
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some(date_today().format("%F").to_string()),
            due_date: Some(date_today().format("%F").to_string()),
        };
        assert_eq!(input, want);
    }

    #[test]
    fn can_undo_marking_task_as_done() {
        let mut input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some(date_today().format("%F").to_string()),
            due_date: Some(date_today().format("%F").to_string()),
        };
        input.mark_undone();
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some(date_today().format("%F").to_string()),
        };
        assert_eq!(input, want);
    }

    #[test]
    fn can_prioritise_a_task() {
        let mut input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        input.prioritise(Some("A".to_string()));
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: Some("A".to_string()),
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        assert_eq!(input, want);
    }

    #[test]
    fn can_remove_task_priority() {
        let mut input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: Some("A".to_string()),
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        input.prioritise(None);
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(input, want);
    }
}
