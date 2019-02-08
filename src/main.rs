extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::env;

mod modify;
mod utility;
mod view;

const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters literal text, not regex.

Modifying:
    a TEXT...               Add a task (add)
    rm IDX                  Remove item IDX
    do IDX                  Move item IDX to $DONEFILE (done)
    undo IDX                Move item IDX from $DONEFILE into $TODOFILE
    up IDX                  Upgrade task IDX to a priority
    down IDX                Downgrade task IDX to a normal task
    app IDX TEXT...         Append TEXT... to task IDX (append)
    repeat IDX [DATE]       Mark task IDX as done, and re-enter in todos (optional due date)
    schedule IDX [DATE]     Schedule task IDX.  If no date, will prompt.
    unschedule IDX          Remove due date from task IDX.
    today IDX               Schedule task IDX for today

Viewing:
    ls               List tasks (optionally filtered)
    lsp              List prioritised tasks (optionally filtered)
    lsd              List done tasks (listdone)
    p                List all unique projects '#PROJECT' (projects)
    pv               List all tasks, grouped by project (projectview)
    pl               List all tasks WITHOUT a project (projectless)
    mit              PRIORITY tasks overdue, or due today
    due [NDAYS]      Show overdue, due today, and tasks due in NDAYS
    nd               Show all todos without a due date (nodate)

Other:
    help             Display this message
";

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let cmd: String = env::args().skip(1).take(1).collect();
    let args: Vec<String> = env::args().skip(2).collect();

    let todos = utility::get_todos(true)?;
    let dones = utility::get_done()?;

    let res = match &cmd[..] {
        // ========== Modification
        "a" | "add" => modify::add(&args),
        "rm" => modify::remove(&args),
        "do" | "done" => modify::do_task(&args),
        "undo" => modify::undo(&args),
        "app" | "append" => modify::append(&args),
        "up" | "upgrade" => modify::prioritise::upgrade(&args),
        "down" | "downgrade" => modify::prioritise::downgrade(&args),
        "repeat" => modify::repeat_task(&args),
        "schedule" => modify::schedule::schedule(&args),
        "unschedule" => modify::schedule::unschedule(&args),
        "today" => modify::schedule::today(&args),
        // ========== Filtered views
        "ls" | "list" => view::list(&todos, &args),
        "lsp" => view::list_priorities(&todos, &args),
        "lsd" | "listdone" => view::done(&dones, &args),
        "p" | "projects" => view::project::projects(&todos, &args),
        "pl" | "projectless" => view::project::projectless(&todos, &args),
        "pv" | "projectview" => view::project::project_view(&todos, &args),
        "due" => view::dated::due(&todos, &args),
        "nd" | "nodate" => view::dated::no_date(&todos, &args),
        "mit" | "important" => view::dated::mit(&todos, &args),
        // ========== Utility
        "filename" => utility::print_todo_filename(),
        "help" => {
            println!("{}", USAGE);
            Ok(())
        }
        _ => {
            if !cmd.is_empty() {
                println!(
                    "Command `{}` unknown.  Defaulting to list (see help or shorthelp)\n",
                    cmd
                );
            }
            view::list(&todos, &args)
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
