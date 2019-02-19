extern crate chrono;
extern crate regex;

use std::env;

mod modify;
mod priority;
mod schedule;
mod todo;
mod utility;
mod view;

const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters will either SHOW filter (+) and/or HIDE filter (-).
Filters apply to any of the VIEW commands.

COMMANDS:
    a TEXT...               Add a task (add)
    app IDX TEXT...         Append TEXT... to task IDX (append)

    rm IDX                  Remove item IDX
    do IDX                  Move item IDX to $DONEFILE (done)
    undo IDX                Move item IDX from $DONEFILE into $TODOFILE

    up IDX                  Upgrade task IDX to a priority
    down IDX                Downgrade task IDX to a normal task

    schedule IDX [DATE]     Schedule task IDX.  If no date, will prompt.
    unschedule IDX          Remove due date from task IDX.
    today IDX               Schedule task IDX for today

    // VIEW commands
    ls                      Show tasks (optionally filtered)
    lsp                     Show prioritised tasks (optionally filtered)
    lsd                     Show done tasks (listdone)
    mit                     Show overdue, or due today, prioritised tasks
    due [NDAYS]             Show overdue, due today, and tasks due in NDAYS
    nd                      Show all todos without a due date (nodate)

    help                    Display this message
";

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let cmd: String = env::args().skip(1).take(1).collect();
    let args: Vec<String> = env::args().skip(2).collect();

    let todos = utility::get::todos()?;
    let dones = utility::get::dones()?;

    let n_todos = todos.len();
    let n_done = dones.len();

    let res = match &cmd[..] {
        // ========== Modification
        "a" | "add" => modify::add(&args),
        "rm" => modify::remove(&args),
        "do" | "done" => modify::do_task(&args),
        "undo" => modify::undo(&args),
        "app" | "append" => modify::append(&args),
        "pre" | "prepend" => modify::prepend(&args),
        "up" | "upgrade" => priority::upgrade(&args),
        "down" | "downgrade" => priority::downgrade(&args),
        "schedule" => schedule::schedule(&args),
        "unschedule" => schedule::unschedule(&args),
        "today" => schedule::today(&args),
        // ========== Filtered views
        "ls" | "list" => view::list(&todos, &args),
        "lsp" => view::list_priorities(&todos, &args),
        "lsd" | "listdone" => view::done(&dones, &args),
        // ========== Date-based views
        "due" => view::dated::due(&todos, &args),
        "nd" | "nodate" => view::dated::no_date(&todos, &args),
        "mit" | "important" => view::dated::mit(&todos, &args),
        // ========== Utility
        _ => {
            println!("{}", USAGE);
            Ok(())
        }
    };

    if res.is_err() {
        println!("{}", res.unwrap_err().description());
        std::process::exit(1);
    }

    if n_todos != 0 && utility::get::todos()?.is_empty() {
        println!("TODOFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    if n_done != 0 && utility::get::dones()?.is_empty() {
        println!("DONEFILE now empty");
        println!("If unexpected, revert using dropbox or git");
    }
    Ok(())
}
