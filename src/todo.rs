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
        utility::notify("APPENDED", self.idx, &self.to_string());
    }

    pub fn prepend_text(&mut self, text: &str) {
        self.task = format!("{} {}", text, self.task);
        utility::notify("PREPENDED", self.idx, &self.task);
    }

    pub fn prioritise(&mut self, priority: Option<String>) {
        self.pri = priority;
        if self.pri.is_some() {
            utility::notify("PRIORITISED", self.idx, &self.task);
        } else {
            utility::notify("DEPRIORITISED", self.idx, &self.task);
        }
    }

    pub fn mark_done(&mut self) {
        self.done_date = Some(utility::date_today().format("%Y-%m-%d").to_string());
        utility::notify("DONE", self.idx, &self.task);
    }

    pub fn mark_undone(&mut self) {
        self.done_date = None;
        utility::notify("UNDONE", self.idx, &self.task);
    }

    pub fn schedule(&mut self, date: &str) {
        self.due_date = Some(utility::parse_date_string_relative(
            utility::date_today(),
            date,
        ));
        utility::notify("SCHEDULED", self.idx, &self.task);
    }

    pub fn unschedule(&mut self) {
        self.due_date = None;
        utility::notify("UNSCHEDULED", self.idx, &self.task);
    }

    pub fn format_for_save(&self) -> String {
        let mut to_output: Vec<String> = vec![];

        if let Some(done) = &self.done_date {
            to_output.push(format!("x {}", done))
        };

        if let Some(p) = self.pri.as_ref() {
            to_output.push(format!("({})", p));
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

        let pre = match self.pri.as_deref() {
            Some("A") => to_colour.yellow().to_string(),
            Some("B") => to_colour.green().to_string(),
            Some("C") => to_colour.blue().to_string(),
            _ => to_colour,
        };

        let mut to_output = vec![pre];

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

#[allow(dead_code, unused_imports)]
mod tests {
    use crate::{todo::Todo, utility::date_today};

    #[test]
    fn test_add() {
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

    #[test]
    fn test_append() {
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
    fn test_prepend() {
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
    fn test_schedule() {
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
    fn test_unschedule() {
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
    fn test_match() {
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
    fn mark_done() {
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
    fn mark_undone() {
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
    fn mark_prioritise() {
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
    fn mark_deprioritise() {
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
