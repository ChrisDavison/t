use super::{todo, utility};

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

pub fn add(text: &[String], todos: &mut Vec<todo::Todo>) -> Result<()> {
    let mut todo: todo::Todo = text.join(" ").parse()?;
    utility::notify("ADDED", todos.len(), &todo.task);
    todo.idx = todos.len();
    Ok(())
}

pub fn append(idx: usize, todos: &mut Vec<todo::Todo>, text: &[String]) -> Result<()> {
    let msg: String = text.iter().skip(1).cloned().collect();
    let n_todos = todos.len();
    if let Some(t) = todos.get_mut(idx) {
        t.append_text(&msg);
        utility::notify("APPENDED", idx, &t.task);
        Ok(())
    } else {
        Err(format!("IDX must be < {} (num todos) - got {}", n_todos, idx).into())
    }
}

pub fn prepend(idx: usize, todos: &mut Vec<todo::Todo>, text: &[String]) -> Result<()> {
    let msg: String = text.iter().skip(1).cloned().collect();
    let n_todos = todos.len();
    if let Some(t) = todos.get_mut(idx) {
        t.prepend_text(&msg);
        utility::notify("PREPENDED", idx, &t.task);
        Ok(())
    } else {
        Err(format!("IDX must be < {} (num todos) - got {}", n_todos, idx).into())
    }
}

pub fn remove(idx: &mut Vec<usize>, todos: &mut Vec<todo::Todo>) -> Result<()> {
    idx.sort_unstable();
    for &i in idx.iter().rev() {
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

    Ok(())
}

pub fn do_task(indices: &mut Vec<usize>, todos: &mut Vec<todo::Todo>) -> Result<()> {
    indices.sort_unstable();
    for &i in indices.iter().rev() {
        if i >= todos.len() {
            continue;
        }
        todos[i].mark_done();
    }
    Ok(())
}

pub fn undo(
    indices: &mut Vec<usize>,
    todos: &mut Vec<todo::Todo>,
    dones: &mut Vec<todo::Todo>,
) -> Result<()> {
    indices.sort_unstable();
    for &i in indices.iter().rev() {
        if i >= dones.len() {
            return Err(From::from("IDX must be within range of num done"));
        }
        let mut done = dones[i].clone();
        done.mark_undone();
        utility::notify("UNDONE", todos.len(), &done.task);
        todos.push(done);
        dones.remove(i);
    }
    Ok(())
}
