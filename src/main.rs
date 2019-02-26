extern crate chrono;
extern crate regex;

use std::env;

mod modify;
mod schedule;
mod todo;
mod utility;
mod view;

const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters only apply to viewing (not modification) commands.  They are
case-insensitive, and will show files matching all `+` and no `-` filters.

Note:
    TODO refers to variable defined by '$TODOFILE'
    DONE refers to variable defined by '$DONEFILE'
    IDX refers to the number of the task you wish to modify
    Words in square brackets are short-aliases

Commands:
    add TEXT...             [a] Add a task
    append IDX TEXT...      [app] Append TEXT... to task
    prepend IDX TEXT...     [pre] Prepend TEXT... to task
    remove IDX              [rm] Remove task
    do IDX                  Move task to DONE
    undo IDX                Move task from DONE to TODO
    
    schedule IDX [DATE]     Schedule task
    unschedule IDX          Remove due date from task
    today IDX               Schedule task for today

    list                    [ls] View tasks 
    listdone                [lsd] View done tasks
    due                     View scheduled tasks
    nodate                  [nd] View unscheduled tasks
    help                    View this message
";

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let cmd: String = env::args().skip(1).take(1).collect();
    let args: Vec<String> = env::args().skip(2).collect();

    let todos = utility::get_todos()?;
    let dones = utility::get_dones()?;

    let n_todos = todos.len();
    let n_done = dones.len();

    let res = match &cmd[..] {
        // ========== Modification
        "a" | "add" => modify::add(&args),
        "rm" | "remove" => modify::remove(&args),
        "do" => modify::do_task(&args),
        "undo" => modify::undo(&args),
        "app" | "append" => modify::append(&args),
        "pre" | "prepend" => modify::prepend(&args),
        "schedule" => schedule::schedule(&args),
        "unschedule" => schedule::unschedule(&args),
        "today" => schedule::today(&args),
        // ========== Filtered views
        "ls" | "list" => view::list(&todos, &args),
        "lsd" | "listdone" => view::done(&dones, &args),
        // ========== Date-based views
        "due" => view::due(&todos, &args),
        "nd" | "nodate" => view::no_date(&todos, &args),
        // ========== Utility
        _ => {
            print!("{}", USAGE);
            Ok(())
        }
    };

    if res.is_err() {
        println!("{}", res.unwrap_err().description());
        std::process::exit(1);
    }

    if n_todos != 0 && utility::get_todos()?.is_empty() {
        println!("TODOFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    if n_done != 0 && utility::get_dones()?.is_empty() {
        println!("DONEFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    Ok(())
}
