use super::{todo, utility};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn add(text: &str, todos: &mut Vec<todo::Todo>) -> Result<()> {
    let mut todo: todo::Todo = text.parse()?;
    todo.idx = todos.len();
    utility::notify("ADDED", todos.len(), &todo.task);
    todos.push(todo);
    Ok(())
}

pub fn append(idx: usize, todos: &mut Vec<todo::Todo>, text: &str) -> Result<()> {
    let n_todos = todos.len();
    if let Some(t) = todos.get_mut(idx) {
        t.append_text(text);
        Ok(())
    } else {
        Err(format!("IDX must be < {} (num todos) - got {}", n_todos, idx).into())
    }
}

pub fn prepend(idx: usize, todos: &mut Vec<todo::Todo>, text: &str) -> Result<()> {
    let n_todos = todos.len();
    if let Some(t) = todos.get_mut(idx) {
        t.prepend_text(text);
        Ok(())
    } else {
        Err(format!("IDX must be < {} (num todos) - got {}", n_todos, idx).into())
    }
}

pub fn prioritise(idx: usize, todos: &mut Vec<todo::Todo>, priority: Option<String>) -> Result<()> {
    let n_todos = todos.len();
    if let Some(t) = todos.get_mut(idx) {
        t.prioritise(priority);
        Ok(())
    } else {
        Err(format!("IDX must be < {} (num todos) - got {}", n_todos, idx).into())
    }
}

pub fn remove(indices: &[usize], todos: &mut Vec<todo::Todo>) -> Result<()> {
    // reverse so that we always pop from the end of the list
    for &i in indices.iter().rev() {
        if i >= todos.len() {
            continue;
        }
        utility::notify("REMOVED", i, &todos[i].task);
        todos.remove(i);
    }
    Ok(())
}

pub fn archive(todos: &mut Vec<todo::Todo>, dones: &mut Vec<todo::Todo>) -> Result<()> {
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

pub fn do_task(indices: &[usize], todos: &mut Vec<todo::Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if i >= todos.len() {
            continue;
        }
        todos[i].mark_done();
    }
    Ok(())
}

pub fn undo(
    indices: &[usize],
    todos: &mut Vec<todo::Todo>,
    dones: &mut Vec<todo::Todo>,
) -> Result<()> {
    for &i in indices.iter().rev() {
        if i >= dones.len() {
            return Err(From::from("IDX must be within range of num done"));
        }
        let mut done = dones[i].clone();
        done.mark_undone();
        todos.push(done);
        dones.remove(i);
    }
    Ok(())
}

pub fn unschedule_each(indices: &[usize], todos: &mut Vec<todo::Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if i >= todos.len() {
            continue;
        }
        todos[i].due_date = None;
        todos[i].unschedule();
    }
    Ok(())
}

pub fn schedule_each_today(indices: &[usize], todos: &mut Vec<todo::Todo>) -> Result<()> {
    for &i in indices.iter().rev() {
        if i >= todos.len() {
            continue;
        }
        todos[i].schedule("today");
    }
    Ok(())
}

pub fn schedule(idx: usize, todos: &mut Vec<todo::Todo>, date: &str) -> Result<()> {
    todos[idx].schedule(date);
    Ok(())
}
