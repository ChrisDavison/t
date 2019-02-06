extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod modify;
mod utility;
mod view;

const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters literal text, not regex.

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
    ls               List tasks (optionally filtered)
    lsp              List prioritised tasks (optionally filtered)
    p                List all unique projects '#PROJECT' (projects)
    pv               List all tasks, grouped by project (projectview)
    pl               List all tasks WITHOUT a project (projectless)
    mit              PRIORITY tasks overdue, or due today
    done             List done tasks
    due [NDAYS]      Show overdue, due today, and tasks due in NDAYS
    nd               Show all todos without a due date (nodate)

Other:
    help             Display this message
";

type Result<T> = ::std::result::Result<T, Box<::std::error::Error>>;

fn main() -> Result<()> {
    let (cmd, positives, negatives, args) = utility::parse_args();

    let todos = utility::filter_todos(&utility::get_todos(true)?, &positives, &negatives);
    let dones = utility::filter_todos(&utility::get_done()?, &positives, &negatives);

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
        "unschedule" => modify::unschedule(&args),
        "today" => modify::today(&args),
        // ========== Filtered views
        "ls" | "list" => view::list(&todos, &args),
        "lsp" => view::list_priorities(&todos),
        "done" => view::done(&dones),
        "p" | "projects" => view::projects(&todos),
        "pl" | "projectless" => view::projectless(&todos),
        "due" => view::due(&todos),
        "nd" | "nodate" => view::no_date(&todos),
        "pv" | "projectview" => view::project_view(&todos),
        "mit" | "important" => view::mit(&todos),
        // ========== Utility
        "filename" => utility::print_todo_filename(),
        "help" => {
            println!("{}", USAGE);
            Ok(())
        },
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
