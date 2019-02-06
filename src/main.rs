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
        "c" | "contexts" => view::contexts(),
        "p" | "projects" => view::projects(),
        "cl" | "contextless" => view::contextless(),
        "pl" | "projectless" => view::projectless(),
        "due" => view::due(),
        "nd" | "nodate" => view::no_date(),
        "pv" | "projectview" => view::project_view(),
        "cv" | "contextview" => view::context_view(),
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
    a,add TEXT...           Add a task
    rm IDX                  Remove item IDX
    do IDX                  Move item IDX to $DONEFILE
    undo IDX                Move item IDX from $DONEFILE into $TODOFILE
    up IDX                  Upgrade task IDX to a priority
    down IDX                Downgrade task IDX to a normal task
    app,append IDX TEXT...  Append TEXT... to task IDX
    repeat IDX              copy task IDX to done, but leave in tasks (e.g. repeat)
    schedule IDX [DATE]     Schedule task IDX.  If no date, will prompt.
    today IDX               Schedule task IDX for today

Viewing:
    ls [QUERY]       List tasks (optionally filtered)
    lsp              List prioritised tasks (optionally filtered)
    hide QUERY       List notes NOT matching query
    c,contexts       List all unique contexts '+CONTEXT'
    p,projects       List all unique projects '@PROJECT'
    pv               List all tasks, grouped by project
    cv               List all tasks, grouped by context
    mit              PRIORITY tasks overdue, or due today

Filtered views:
    cl,contextless            Tasks without a context
    pl,projectless            Tasks without a project
    done                      List done tasks
    filename                  Alias to print $TODOFILE
    due [NDAYS]               Show overdue, due today, and tasks due in NDAYS
    nd,nodate                 Show all todos without a due date

Other:
    help             Display short help message (most common commands)
    longhelp         Display this help message
"
    );
    Ok(())
}
