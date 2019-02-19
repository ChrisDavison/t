use super::utility;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn change_priority(args: &[String], new: bool) -> Result<()> {
    let idx: usize = match args.get(0) {
        Some(i) => i.parse()?,
        None => return Err(From::from("Must pass IDX argument")),
    };
    let mut todos = utility::get::todos()?;
    if todos.len() < idx {
        return Err(From::from(format!(
            "IDX must be < {} (number of tasks)",
            todos.len()
        )));
    }
    todos[idx].priority = new;
    let msg = if new { "UPGRADED" } else { "DOWNGRADED" };
    println!("{} {} {}", msg, idx, todos[idx].task);
    utility::save::todos(&todos)
}

pub fn upgrade(args: &[String]) -> Result<()> {
    change_priority(&args, true)
}

pub fn downgrade(args: &[String]) -> Result<()> {
    change_priority(&args, false)
}
