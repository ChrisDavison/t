use chrono::{Date, NaiveDate, Utc};

use super::{colour, utility};
use std::fmt::{self, Display};
use std::str::FromStr;

// use itertools::Itertools;
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Todo {
    pub idx: usize,
    pub task: String,
    pub pri: TodoPriority,
    pub projects: Vec<String>,
    pub contexts: Vec<String>,
    pub done_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum TodoPriority {
    A,
    B,
    C,
    None,
}

impl Display for TodoPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoPriority::A => write!(f, "(A)"),
            TodoPriority::B => write!(f, "(B)"),
            TodoPriority::C => write!(f, "(C)"),
            _ => write!(f, ""),
        }
    }
}

impl Todo {
    fn case_insensitive_match(haystack: &impl ToString, needle: &impl ToString) -> bool {
        haystack
            .to_string()
            .to_ascii_lowercase()
            .contains(&needle.to_string().to_ascii_lowercase())
    }

    pub fn matches(&self, positives: &[impl ToString], negatives: &[impl ToString]) -> bool {
        let taskstr = format!(
            "{}{}{}",
            self.task,
            &self.projects.join(" "),
            &self.contexts.join(" ")
        );
        let has_all_pos = positives
            .iter()
            .all(|y| Todo::case_insensitive_match(&taskstr, y));
        let has_no_neg = !negatives
            .iter()
            .any(|y| Todo::case_insensitive_match(&taskstr, y));
        has_all_pos && has_no_neg
    }

    pub fn append_text(&mut self, text: &str) {
        self.task = format!("{} {}", self.task, text);
        // self.task.push_str(text);
        utility::notify("APPENDED", &self);
    }

    pub fn prepend_text(&mut self, text: &str) {
        self.task = format!("{} {}", text, self.task);
        utility::notify("PREPENDED", &self);
    }

    pub fn prioritise(&mut self, priority: TodoPriority) {
        self.pri = priority;
        if !matches!(self.pri, TodoPriority::None) {
            utility::notify("PRIORITISED", &self);
        } else {
            utility::notify("DEPRIORITISED", self);
        }
    }

    pub fn mark_done(&mut self) {
        self.done_date = Some(utility::date_today().format("%Y-%m-%d").to_string());
        self.pri = TodoPriority::None;
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

    pub fn days_overdue(&self) -> super::Result<i64> {
        let now: Date<Utc> = utility::date_today();
        let naive = NaiveDate::parse_from_str(
            self.due_date.as_ref().unwrap_or(&String::from("")).as_ref(),
            "%Y-%m-%d",
        )
        .map_err(|e| anyhow::anyhow!("Couldn't parse date {:#?}: {}", self, e))?;
        let task_date = Date::from_utc(naive, *now.offset());
        Ok((now - task_date).num_days())
    }

    pub fn days_since_done(&self) -> super::Result<i64> {
        let now: Date<Utc> = utility::date_today();
        let naive = NaiveDate::parse_from_str(
            self.done_date
                .as_ref()
                .unwrap_or(&String::from(""))
                .as_ref(),
            "%Y-%m-%d",
        )
        .map_err(|e| anyhow::anyhow!("Couldn't parse date {:#?}: {}", self, e).to_string())?;
        let task_date = Date::from_utc(naive, *now.offset());
        Ok((now - task_date).num_days())
    }

    #[inline(always)]
    fn done_or_priority_string(&self) -> String {
        match (&self.done_date, self.pri) {
            (Some(done), _) => format!("x {}", done),
            (None, prio) => format!("{}", prio),
        }
    }

    // display [x DONEDATE | PRIORITY] TEXT [DUEDATE] +TAGS @CONTEXTS
    pub fn format_for_save(&self) -> String {
        utility::join_non_empty(
            [
                &self.done_or_priority_string(),
                &self.task,
                &self
                    .due_date
                    .as_ref()
                    .map(|x| format!("due:{}", x))
                    .unwrap_or_default(),
                &self.projects.join(" "),
                &self.contexts.join(" "),
            ]
            .iter(),
        )
    }

    // display TEXT +TAGS @CONTEXTS
    pub fn donesummary_format(&self) -> String {
        utility::join_non_empty(
            [
                &self.task,
                &self.projects.join(" "),
                &self.contexts.join(" "),
            ]
            .iter(),
        )
    }
}

// Implement .parse() for Todo
impl FromStr for Todo {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut done_date = None;
        let mut priority = TodoPriority::None;
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
                priority = match &token[1..2] {
                    "A" => TodoPriority::A,
                    "B" => TodoPriority::B,
                    "C" => TodoPriority::C,
                    _ => TodoPriority::None,
                }
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

impl Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pre = utility::join_non_empty(
            [
                &self.done_or_priority_string(),
                &self.task,
                &self
                    .due_date
                    .as_ref()
                    .map(|x| format!("due:{}", x))
                    .unwrap_or_default(),
            ]
            .iter(),
        );
        let post =
            utility::join_non_empty([&self.projects.join(" "), &self.contexts.join(" ")].iter());

        let colourer = match self.pri {
            TodoPriority::A => colour::yellow,
            TodoPriority::B => colour::green,
            TodoPriority::C => colour::blue,
            _ => colour::none,
        };

        let (pre, post) = if colour::should_colour() {
            (colourer(&pre), colour::red(&post))
        } else {
            (pre, post)
        };

        write!(f, "{:3}. {} {}", self.idx, pre, post)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        todo::{Todo, TodoPriority},
        utility::date_today,
    };

    #[test]
    fn can_display_task() {
        let input = "this is a test +p1 +p2 @c1";
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        assert_eq!(format!("  0. {}", input), t.to_string());
    }

    #[test]
    fn can_parse_task() {
        let input = "this is a test +p1 +p2 @c1";
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        let got: Todo = input.parse().unwrap();
        assert_eq!(t, got);
    }

    #[test]
    fn can_parse_done_task() {
        let input = "x 2021-01-01 this is a test +p1 +p2 @c1";
        let t = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.append_text("EXTRA");

        let expected = Todo {
            idx: 0,
            task: "this is a test EXTRA".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.prepend_text("EXTRA");

        let expected = Todo {
            idx: 0,
            task: "EXTRA this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        t.schedule("today");

        let expected = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        t.unschedule();

        let expected = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some(date_today().format("%F").to_string()),
        };
        input.mark_done();
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some(date_today().format("%F").to_string()),
            due_date: Some(date_today().format("%F").to_string()),
        };
        assert_eq!(input, want);

        let mut input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::A,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some(date_today().format("%F").to_string()),
        };
        input.mark_done();
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some(date_today().format("%F").to_string()),
            due_date: Some(date_today().format("%F").to_string()),
        };
        input.mark_undone();
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
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
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        };
        input.prioritise(TodoPriority::A);
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::A,
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
            pri: TodoPriority::A,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        input.prioritise(TodoPriority::None);
        let want = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(input, want);
    }

    #[test]
    fn can_format_for_saving() {
        let input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::A,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(
            input.format_for_save(),
            "(A) this is a test due:2021-01-01 +p1 +p2 @c1"
        );

        let input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some("2021-01-01".to_string()),
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(
            input.format_for_save(),
            "x 2021-01-01 this is a test due:2021-01-01 +p1 +p2 @c1"
        );

        let input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec![],
            contexts: vec!["@c1".to_string()],
            done_date: Some("2021-01-01".to_string()),
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(
            input.format_for_save(),
            "x 2021-01-01 this is a test due:2021-01-01 @c1"
        );
    }

    #[test]
    fn can_format_todo_output() {
        let input = Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: TodoPriority::None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: Some("2021-01-01".to_string()),
            due_date: Some("2021-01-01".to_string()),
        };
        assert_eq!(
            "  0. x 2021-01-01 this is a test due:2021-01-01 +p1 +p2 @c1",
            input.to_string(),
        );
    }
}
