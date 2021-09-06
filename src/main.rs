use structopt::StructOpt;

mod modify;
mod schedule;
mod todo;
mod utility;
mod view;

#[derive(StructOpt, Debug)]
// #[structopt(name="t", setting=AppSettings::InferSubcommands)]
enum Command {
    /// Add a task
    #[structopt(alias = "a")]
    Add { text: Vec<String> },
    /// Append text to a task
    #[structopt(alias = "app")]
    Append { idx: usize, text: Vec<String> },
    /// Prepend text to a task
    #[structopt(alias = "pre")]
    Prepend { idx: usize, text: Vec<String> },
    /// Remove a task
    #[structopt(alias = "rm")]
    Remove { idxs: Vec<usize> },
    /// Move task to DONEFILE
    Do { idx: Vec<usize> },
    /// Move task from DONEFILE to TODOFILE
    Undo { idx: Vec<usize> },
    /// Schedule a task
    Schedule { idx: Vec<usize>, date: String },
    /// Remove due date from task
    Unschedule { idx: Vec<usize> },
    /// Schedule task today
    Today { idx: Vec<usize> },
    /// View tasks
    #[structopt(alias = "ls")]
    List { filters: Vec<String> },
    /// View tasks with a priority
    #[structopt(alias = "lsp")]
    ListPriority { filters: Vec<String> },
    /// View done tasks
    #[structopt(alias = "lsd")]
    ListDone { filters: Vec<String> },
    /// View scheduled tasks
    Due { filters: Vec<String> },
    /// View unscheduled tasks
    NoDate { filters: Vec<String> },
}

#[allow(dead_code)]
const USAGE: &str = "usage: t <CMD> [+filter...] [-filter...] [ARGS...]

Filters only apply to viewing (not modification) commands.  They are
case-insensitive, and will show todos matching all `+` and no `-` filters.

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
    listdone                [lsd|done] View done tasks
    listpriority            [lsp] View tasks with a priority
    due                     View scheduled tasks
    nodate                  [nd] View unscheduled tasks
    help                    View this message
";

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    let opts = Command::from_args();

    let todos = match utility::get_todos() {
        Ok(todos) => todos,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    let dones = match utility::get_dones() {
        Ok(dones) => dones,
        Err(e) => {
            println!("{}", e);
            std::process::exit(2);
        }
    };

    // let num_todos_at_start = todos.len();
    // let num_done_at_start = dones.len();

    let result = match opts {
        // ========== Modification
        Command::Add { text } => modify::add(&text),
        Command::Append { idx, text } => modify::append(idx, &text),
        Command::Prepend { idx, text } => modify::prepend(idx, &text),
        Command::Remove { idxs } => modify::remove(&idxs),
        Command::Do { idx } => modify::do_task(&idx),
        Command::Undo { idx } => modify::undo(&idx),
        // ========== SCHEDULING
        Command::Schedule { idx, date } => schedule::schedule(&idx, &date),
        Command::Unschedule { idx } => schedule::unschedule(&idx),
        Command::Today { idx } => schedule::today(&idx),
        // ========== Filtered views
        Command::List { filters } => view::list(&todos, &filters),
        Command::ListPriority { filters } => view::list_priority(&todos, &filters),
        Command::ListDone { filters } => view::done(&dones, &filters),
        // ========== Date-based views
        Command::Due { filters } => view::due(&todos, &filters),
        Command::NoDate { filters } => view::no_date(&todos, &filters),
        // ========== Date-based views
    };

    if let Err(err) = result {
        println!("{}", err.to_string());
        std::process::exit(1);
    }

    // if num_todos_at_start != 0 && utility::get_todos()?.is_empty() {
    //     println!("TODOFILE is now empty");
    // }
    // if num_done_at_start != 0 && utility::get_dones()?.is_empty() {
    //     println!("DONEFILE is now empty");
    // }
    Ok(())
}
