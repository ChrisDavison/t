extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod modify;
mod utility;
mod view;

const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters will either SHOW filter (+) and/or HIDE filter (-).

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

    let todos = utility::get_todos()?;
    let dones = utility::get_dones()?;

    let res = match &cmd[..] {
        // ========== Modification
        "a" | "add" => modify::add(&args),
        "rm" => modify::remove(&args),
        "do" | "done" => modify::do_task(&args),
        "undo" => modify::undo(&args),
        "app" | "append" => modify::append(&args),
        "pre" | "prepend" => modify::prepend(&args),
        "up" | "upgrade" => modify::prioritise::upgrade(&args),
        "down" | "downgrade" => modify::prioritise::downgrade(&args),
        "schedule" => modify::schedule::schedule(&args),
        "unschedule" => modify::schedule::unschedule(&args),
        "today" => modify::schedule::today(&args),
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

    let res = utility::check_for_blank_files();
    if res.is_err() {
        println!("{}", res.unwrap_err().description());
        std::process::exit(2);
    }
    Ok(())
}
