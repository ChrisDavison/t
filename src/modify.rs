use super::{todo, utility};

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get::todos()?;
    let todo = todo::parse_todo(todos.len(), &args.join(" ").to_string());
    println!("ADDED {} {}", todos.len(), todo.task);
    todos.push(todo);
    utility::save::todos(&todos)
}

pub fn append(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t app IDX TEXT...")),
    };
    let mut todos = utility::get::todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", new.task, msg);
    println!("APPENDED {} {}", idx, &new.task);
    todos[idx] = new.clone();
    utility::save::todos(&todos)
}

pub fn prepend(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("usage: t app IDX TEXT...")),
    };
    let mut todos = utility::get::todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    let mut new = &mut todos[idx];
    new.task = format!("{} {}", msg, new.task);
    println!("PREPENDED {} {}", idx, &new.task);
    todos[idx] = new.clone();
    utility::save::todos(&todos)
}

pub fn remove(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t rm IDX"));
    }
    let mut todos = utility::get::todos()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    println!("REMOVED {} {}", idx, &todos[idx].task);
    todos.remove(idx);
    utility::save::todos(&todos)
}

pub fn do_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get::todos()?;
    let mut dones = utility::get::dones()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    let mut done_task = todos[idx].clone();
    done_task.done = utility::get_formatted_date();
    println!("COMPLETE {} {} {}", idx, done_task.done, &todos[idx].task);
    dones.push(done_task.clone());
    todos.remove(idx);

    utility::save::todos(&todos)?;
    utility::save::dones(&dones)
}

pub fn undo(args: &[String]) -> Result<()> {
    let mut todos = utility::get::todos()?;
    let mut dones = utility::get::dones()?;
    let (idx, msg) = if args.is_empty() {
        (dones.len() - 1, "UNDONE LAST")
    } else {
        (args[0].parse()?, "UNDONE")
    };
    if idx >= dones.len() {
        return Err(From::from("IDX must be within range of num done"));
    }
    let mut done = dones[idx].clone();
    done.idx = todos.len();
    println!("{} {} {}", msg, todos.len(), &done.task);
    todos.push(done);
    dones.remove(idx);

    utility::save::todos(&todos)?;
    utility::save::dones(&dones)
}
