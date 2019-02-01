extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;

mod view;
mod modify;
mod utility;

const USAGE: &str = "usage: t <CMD> [ARGS...]

View-queries are literal, not regex.

Modifying:
    a|add TEXT...       Add a task
    rm #NUM             Remove item #NUM
    do #NUM             Mark item #NUM as done
    undo #NUM           Move item #NUM from DONEFILE into TODOFILE
    up #NUM             Upgrade task #NUM to a priority
    down #NUM           Downgrade task #NUM to a normal task
    append IDX TEXT...  Append TEXT... to task IDX

Viewing:
    ls [QUERY]       List tasks (optionally filtered)
    lsp [QUERY]      List prioritised tasks (optionally filtered)
    hide QUERY       List notes NOT matching query
    c|contexts       List all unique contexts '+CONTEXT'
    p|projects       List all unique projects '@PROJECT'
    pv               List all tasks, grouped by project
    cv               List all tasks, grouped by context
    mit              PRIORITY tasks overdue, or due today

Filtered views:
    cl|contextless            Tasks without a context
    pl|projectless            Tasks without a project
    done                      List done tasks
    filename                  Alias to print $TODOFILE
    due [NDAYS]               Show overdue, due today, and tasks due in NDAYS
";

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
        "pv"|"projectview" => view::project_view(),
        "cv"|"contextview" => view::context_view(),
        "mit"|"important" => view::mit(),
        // ========== Utility
        "filename" => utility::print_todo_filename(),
        _ => Err(From::from(USAGE)),
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
