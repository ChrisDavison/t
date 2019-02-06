extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod modify;
mod utility;
mod view;

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() {
    let cmd: String = env::args().skip(1).take(1).collect();
    let args: Vec<String> = env::args().skip(2).collect();

    let res = match &cmd[..] {
        // ========== Modification
        "a" | "add" => modify::add(&args),
        "rm" => modify::remove(&args),
        "do" => modify::do_task(&args),
        "undo" => modify::undo(&args),
        "app" | "append" => modify::append(&args),
        "up" | "upgrade" => modify::upgrade(&args),
        "down" | "downgrade" => modify::downgrade(&args),
        "cleardone" => modify::clear_done(),
        "repeat" => modify::repeat_task(&args),
        "schedule" => modify::schedule(&args),
        "today" => modify::today(&args),
        // ========== Filtered views
        "ls" | "list" => view::list(&args),
        "lsp" => view::list_priorities(),
        "h" | "hide" => view::hide(&args),
        "done" => view::done(),
        "p" | "projects" => view::projects(),
        "pl" | "projectless" => view::projectless(),
        "due" => view::due(),
        "nd" | "nodate" => view::no_date(),
        "pv" | "projectview" => view::project_view(),
        "mit" | "important" => view::mit(),
        // ========== Utility
        "filename" => utility::print_todo_filename(),
        "help" => short_usage(),
        "lh" | "longhelp" => long_usage(),
        _ => {
            if !cmd.is_empty() {
                println!(
                    "Command `{}` unknown.  Defaulting to list (see help or shorthelp)\n",
                    cmd
                );
            }
            view::list(&args)
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
}

fn short_usage() -> Result<()> {
    println!(
        "usage: t <CMD> [ARGS...]

Most common commands
    a    TEXT...       Add a task
    rm   IDX           Remove item IDX
    do   IDX           Move item IDX to $DONEFILE
    app  IDX TEXT...   Append TEXT... to task IDX
    ls   [QUERY]       List tasks (optionally filtered)
    mit                PRIORITY tasks overdue, or due today
    done               List done tasks
    longhelp           Show ALL commands
"
    );
    Ok(())
}

fn long_usage() -> Result<()> {
    println!(
        "usage: t <CMD> [ARGS...]

View-queries are literal, not regex.

Modifying:
    a TEXT...               Add a task (add)
    rm IDX                  Remove item IDX
    do IDX                  Move item IDX to $DONEFILE
    undo IDX                Move item IDX from $DONEFILE into $TODOFILE
    up IDX                  Upgrade task IDX to a priority
    down IDX                Downgrade task IDX to a normal task
    app IDX TEXT...         Append TEXT... to task IDX (append)
    repeat IDX              copy task IDX to done, but leave in tasks
    schedule IDX [DATE]     Schedule task IDX.  If no date, will prompt.
    today IDX               Schedule task IDX for today

Viewing:
    ls [QUERY]       List tasks (optionally filtered)
    lsp              List prioritised tasks (optionally filtered)
    hide QUERY       List notes NOT matching query
    p                List all unique projects '#PROJECT' (projects)
    pv               List all tasks, grouped by project (projectview)
    pl               List all tasks WITHOUT a project (projectless)
    mit              PRIORITY tasks overdue, or due today
    done             List done tasks
    due [NDAYS]      Show overdue, due today, and tasks due in NDAYS
    nd               Show all todos without a due date (nodate)

Other:
    help             Display short help message (most common commands)
    longhelp         Display this help message
"
    );
    Ok(())
}
