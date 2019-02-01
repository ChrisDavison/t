use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

pub fn add(args: &[String]) -> Result<()> {
    let mut todos = utility::get_todos(false)?;
    let msg = format!("- {}", args.join(" "));
    todos.push((todos.len(), msg));
    utility::write_enumerated_todos(&todos)
}

pub fn append(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    let msg: String = args.iter().skip(1).cloned().collect();
    todos[idx] = (todos[idx].0, format!("{} {}", todos[idx].1, msg));
    utility::write_enumerated_todos(&todos)
}

pub fn remove(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t rm IDX"));
    }
    let mut todos = utility::get_todos(false)?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    println!("REMOVING: {}", &todos[idx].1);
    todos.remove(idx);
    utility::write_enumerated_todos(&todos)
}

pub fn do_task(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t do IDX"));
    }
    let mut todos = utility::get_todos(false)?;
    let mut dones = utility::get_done()?;
    let idx: usize = args[0].parse()?;
    if idx >= todos.len() {
        return Err(From::from("IDX must be within range of num todos"));
    }
    let dated_task = format!("{} done:{}", todos[idx].1, utility::get_formatted_date());
    dones.push((todos[idx].0, dated_task));
    todos.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub fn undo(args: &[String]) -> Result<()> {
    if args.is_empty() {
        return Err(From::from("usage: t undo IDX"));
    }
    let mut todos = utility::get_todos(false)?;
    let mut dones = utility::get_done()?;
    let idx: usize = args[0].parse()?;
    if idx >= dones.len() {
        return Err(From::from("IDX must be within range of num done"));
    }
    todos.push((dones[idx].0, dones[idx].1.clone()));
    dones.remove(idx);

    utility::write_enumerated_todos(&todos)?;
    utility::write_enumerated_dones(&dones)
}

pub fn upgrade(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    if !todos[idx].1.starts_with("- ! ") {
        todos[idx] = (todos[idx].0, format!("- ! {}", &todos[idx].1[2..]));
    }
    utility::write_enumerated_todos(&todos)
}

pub fn downgrade(args: &[String]) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX")),
    };
    let mut todos = utility::get_todos(false)?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    if todos[idx].1.starts_with("- ! ") {
        todos[idx] = (todos[idx].0, format!("- {}", &todos[idx].1[4..]));
    }
    utility::write_enumerated_todos(&todos)
}

pub fn clear_done() -> Result<()> {
    let filename = std::env::var("DONEFILE")?;
    std::fs::write(filename, String::new())?;
    Ok(())
}
