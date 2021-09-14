use super::{todo::Todo, utility};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn add(text: &str, todos: &mut Vec<Todo>) -> Result<()> {
    let mut todo: Todo = text.parse()?;
    todo.idx = todos.len();
    utility::notify("ADDED", &todo);
    todos.push(todo);
    Ok(())
}

pub fn addx(text: &str, todos: &mut Vec<Todo>) -> Result<()> {
    add(text, todos)?;
    do_task(&[todos.len() - 1], todos)
}

pub fn adda(text: &str, todos: &mut Vec<Todo>) -> Result<()> {
    add(text, todos)?;
    prioritise(todos.len() - 1, todos, Some("A".to_string()))
}

pub fn addt(text: &str, todos: &mut Vec<Todo>) -> Result<()> {
    add(text, todos)?;
    schedule(todos.len() - 1, todos, "today")
}

pub fn append(idx: usize, todos: &mut Vec<Todo>, text: &str) -> Result<()> {
    if let Some(t) = todos.get_mut(idx) {
        t.append_text(text);
    }
    Ok(())
}

pub fn prepend(idx: usize, todos: &mut Vec<Todo>, text: &str) -> Result<()> {
    if let Some(t) = todos.get_mut(idx) {
        t.prepend_text(text);
    }
    Ok(())
}

pub fn prioritise(idx: usize, todos: &mut Vec<Todo>, priority: Option<String>) -> Result<()> {
    if let Some(t) = todos.get_mut(idx) {
        t.prioritise(priority);
    }
    Ok(())
}

pub fn remove(indices: &[usize], todos: &mut Vec<Todo>) -> Result<()> {
    // reverse so that we always pop from the end of the list
    for &i in indices.iter().rev() {
        if let Some(t) = todos.get_mut(i) {
            utility::notify("REMOVED", t);
            todos.remove(i);
        }
    }
    Ok(())
}

pub fn schedule(idx: usize, todos: &mut Vec<Todo>, date: &str) -> Result<()> {
    if let Some(t) = todos.get_mut(idx) {
        t.schedule(date);
    }
    Ok(())
}

pub fn archive(todos: &mut Vec<Todo>, dones: &mut Vec<Todo>) -> Result<()> {
    let mut todos_to_pop = Vec::new();
    let mut dones_to_pop = Vec::new();

    // Add DONE _todos_ to DONES
    for (i, todo) in todos.iter().enumerate() {
        if todo.done_date.is_some() {
            dones.push(todo.clone());
            todos_to_pop.push(i);
        }
    }
    // Add UNDONE _dones_ to TODOS
    for (i, done) in dones.iter().enumerate() {
        if done.done_date.is_none() {
            todos.push(done.clone());
            dones_to_pop.push(i);
        }
    }
    // Remove DONE todos
    for idx in todos_to_pop.iter().rev() {
        todos.remove(*idx);
    }
    // Remove UNDONE dones
    for idx in dones_to_pop.iter().rev() {
        dones.remove(*idx);
    }
    if !todos_to_pop.is_empty() {
        println!("Archived {} tasks", todos_to_pop.len());
    }
    if !dones_to_pop.is_empty() {
        println!("Unarchived {} tasks", dones_to_pop.len());
    }

    Ok(())
}

pub fn do_task(indices: &[usize], todos: &mut Vec<Todo>) -> Result<()> {
    indices.iter().rev().for_each(|&idx| {
        if let Some(t) = todos.get_mut(idx) {
            t.mark_done()
        }
    });

    Ok(())
}

pub fn undo(indices: &[usize], todos: &mut Vec<Todo>, dones: &mut Vec<Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if let Some(done) = dones.get_mut(i) {
            done.mark_undone();
            todos.push(done.clone());
            dones.remove(i);
        }
    }
    Ok(())
}

pub fn unschedule_each(indices: &[usize], todos: &mut Vec<Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if let Some(t) = todos.get_mut(i) {
            t.unschedule();
        }
    }
    Ok(())
}

pub fn schedule_each_today(indices: &[usize], todos: &mut Vec<Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if let Some(t) = todos.get_mut(i) {
            t.schedule("today");
        }
    }
    Ok(())
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn can_prioritise_tasks() {
        let mut tasks = vec![Todo {
            idx: 0,
            task: "this is a test".to_string(),
            pri: None,
            projects: vec!["+p1".to_string(), "+p2".to_string()],
            contexts: vec!["@c1".to_string()],
            done_date: None,
            due_date: None,
        }];
        prioritise(0, &mut tasks, Some("A".to_string())).unwrap();
        assert_eq!(tasks[0].pri, Some("A".to_string()))
    }
}
