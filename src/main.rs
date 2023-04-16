extern crate anyhow;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod colour;
mod modify;
mod todo;
mod utility;
mod view;

#[derive(Debug)]
enum Command {
    /// Add a task
    Add { text: String },
    /// Add a task and complete immediately
    Addx { text: String },
    /// Add a task and prioritise as 'A'
    Adda { text: String },
    /// Add a task and schedule today
    Addt { text: String },
    /// Append text to a task
    Append { idx: usize, text: String },
    /// Prepend text to a task
    Prepend { idx: usize, text: String },
    /// Prioritise a task
    Prioritise { idx: usize, priority: String },
    /// Deprioritise a task
    Deprioritise { idx: usize },
    /// Remove a task
    Remove { idxs: Vec<usize> },
    /// Move task to DONEFILE
    Do { idxs: Vec<usize> },
    /// Move task from DONEFILE to TODOFILE
    Undo { idxs: Vec<usize> },
    /// Schedule a task
    Schedule { idx: usize, date: String },
    /// Remove due date from task
    Unschedule { idxs: Vec<usize> },
    /// Schedule task today
    Today { idxs: Vec<usize> },
    /// View tasks
    List { filters: Vec<String> },
    /// View tasks with a priority
    ListPriority { filters: Vec<String> },
    /// View done tasks
    ListDone { filters: Vec<String> },
    /// Projects
    ListProjects,
    /// Contexts
    ListContexts,
    /// View tasks grouped by project
    ProjectView { filters: Vec<String> },
    /// View tasks grouped by context
    ContextView { filters: Vec<String> },
    /// View done tasks, by date, for last N days
    DoneSummary { days: i64, filters: Vec<String> },
    /// View scheduled tasks
    Due { n_days: usize, filters: Vec<String> },
    /// View unscheduled tasks
    NoDate { filters: Vec<String> },
    /// Move done tasks into DONEFILE
    Archive,
}

const USAGE: &str = "usage: t <COMMAND> [ARGS]...

Commands:
    add TEXT...              [a] Add a task
    addx TEXT...             [a] Add a task and complete immediately
    adda TEXT...             [a] Add a task with priority a
    append IDX TEXT...       [app] Append TEXT... to task
    prepend IDX TEXT...      [pre] Prepend TEXT... to task
    priority IDX PRIORITY    [pri] Change priority of task
    deprioritise IDX         [depri|dp] Remove task priority
    remove IDX...            [rm|del] Remove task
    do IDX...                Move task to DONE
    undo IDX...              Move task from DONE to TODO
    schedule IDX DATE        Schedule task
    unschedule IDX...        Remove due date from task
    today IDX...             Schedule task for today

    list [FILTER]...         [ls] View tasks
    listdone [FILTER]...     [lsd|done] View done tasks
    listpriority [FILTER]... [lsp] View tasks with a priority
    listprojects             [prj|lsprj] View projects
    listcontexts             [con|lscon] View contexts
    projectview [FILTER]...  [pv] View tasks grouped by project
    due [FILTER]...          View scheduled tasks
    nodate [FILTER]...       [nd] view unscheduled tasks
    donesummary [FILTER]...  [ds] view completed tasks in last 7 days
    archive                  move done tasks to archive

    help                     View this message

Note:
    Files - $TODOFILE and $DONEFILE, both following todo.txt syntax
    Filters - match NO words beginning with '-'. Match ALL the rest.
    IDX refers to the number of the task you wish to modify.
    Words in square brackets are aliases for commands";

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let mut todos = match utility::get_todos() {
        Ok(todos) => todos,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let mut dones = match utility::get_dones() {
        Ok(dones) => dones,
        Err(e) => {
            println!("{}", e);
            std::process::exit(2);
        }
    };

    let num_todos_at_start = todos.len();
    let num_done_at_start = dones.len();

    debug!("Started with {} todos", num_todos_at_start);
    debug!("Started with {} dones", num_done_at_start);

    let (command, mut autoarchive) = parse_args(num_todos_at_start, num_done_at_start)?;
    debug!("Autoarchiving? {}", autoarchive);

    let result = match command {
        // ========== Modification
        Command::Add { text } => modify::add(&text, &mut todos),
        Command::Addx { text } => modify::addx(&text, &mut todos),
        Command::Adda { text } => modify::adda(&text, &mut todos),
        Command::Addt { text } => modify::addt(&text, &mut todos),
        Command::Append { idx, text } => modify::append(idx, &mut todos, &text),
        Command::Prepend { idx, text } => modify::prepend(idx, &mut todos, &text),
        Command::Prioritise { idx, priority } => {
            modify::prioritise(idx, &mut todos, Some(priority))
        }
        Command::Deprioritise { idx } => modify::prioritise(idx, &mut todos, None),

        Command::Remove { idxs } => modify::remove(&idxs, &mut todos),
        Command::Do { idxs } => modify::do_task(&idxs, &mut todos),
        Command::Undo { idxs } => modify::undo(&idxs, &mut todos, &mut dones),
        // ========== SCHEDULING
        Command::Schedule { idx, date } => modify::schedule(idx, &mut todos, &date),
        Command::Unschedule { idxs } => modify::unschedule_each(&idxs, &mut todos),
        Command::Today { idxs } => modify::schedule_each_today(&idxs, &mut todos),
        // ========== Filtered views
        Command::List { filters } => view::list(todos.iter(), &filters),
        Command::ListPriority { filters } => view::list_priority(todos.iter(), &filters),
        Command::ListDone { filters } => view::done(dones.iter(), &filters),
        Command::DoneSummary { days, filters } => view::done_summary(dones.iter(), &filters, days),
        Command::ListProjects => view::projects(todos.iter()),
        Command::ListContexts => view::contexts(todos.iter()),
        Command::ProjectView { filters } => view::grouped_by_project(todos.iter(), &filters),
        Command::ContextView { filters } => view::grouped_by_context(todos.iter(), &filters),
        // ========== Date-based views
        Command::Due { n_days, filters } => view::due(todos.iter(), n_days, &filters),
        Command::NoDate { filters } => view::no_date(todos.iter(), &filters),
        // ========== Date-based views
        Command::Archive => {
            autoarchive = false;
            modify::archive(&mut todos, &mut dones)
        }
    };

    if let Err(err) = result {
        println!("{}", err.to_string());
        std::process::exit(1);
    }

    if autoarchive {
        if let Err(err) = modify::archive(&mut todos, &mut dones) {
            println!("{}", err.to_string());
            std::process::exit(1);
        }
    }
    utility::save_to_file(todos.iter(), std::env::var("TODOFILE")?)?;
    utility::save_to_file(dones.iter(), std::env::var("DONEFILE")?)?;

    if num_todos_at_start != 0 && todos.is_empty() {
        println!("TODOFILE is now empty");
    }
    if num_done_at_start != 0 && dones.is_empty() {
        println!("DONEFILE is now empty");
    }
    Ok(())
}

// Return the command to execute, and whether to auto-archive
// Also, will filter any user-provided indices to be within range of available
// todos
fn parse_args(n_todos: usize, n_dones: usize) -> Result<(Command, bool)> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    let t_dont_autoarchive_env =
        std::env::var("T_DONT_AUTOARCHIVE").unwrap_or_else(|_| "false".to_string());
    let autoarchive = pargs.contains(["-a", "--autoarchive"])
        || t_dont_autoarchive_env.is_empty()
        || t_dont_autoarchive_env == "false";

    let rest_as_strings = |pargs: pico_args::Arguments| {
        pargs
            .finish()
            .iter()
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<String>>()
    };

    let rest_as_usizes_within_bounds =
        |pargs: pico_args::Arguments, limit: usize| -> Result<Vec<usize>> {
            let mut usizes: Vec<usize> = Vec::new();
            for string in rest_as_strings(pargs) {
                let parsed = string.parse()?;
                if parsed < limit {
                    usizes.push(parsed);
                }
            }
            usizes.sort_unstable();
            Ok(usizes)
        };

    let command = match pargs.subcommand()?.as_deref() {
        Some("help" | "h") => {
            println!("{}", USAGE);
            std::process::exit(0);
        }
        Some("add" | "a") => Command::Add {
            text: rest_as_strings(pargs).join(" "),
        },
        Some("addx") => Command::Addx {
            text: rest_as_strings(pargs).join(" "),
        },
        Some("adda" | "aa") => Command::Adda {
            text: rest_as_strings(pargs).join(" "),
        },
        Some("addt" | "at") => Command::Addt {
            text: rest_as_strings(pargs).join(" "),
        },
        Some("append" | "app") => Command::Append {
            idx: pargs.free_from_str()?,
            text: rest_as_strings(pargs).join(" "),
        },
        Some("prepend" | "pre") => Command::Prepend {
            idx: pargs.free_from_str()?,
            text: rest_as_strings(pargs).join(" "),
        },
        Some("priority" | "pri") => Command::Prioritise {
            idx: pargs.free_from_str()?,
            priority: pargs.free_from_str()?,
        },
        Some("deprioritise" | "depri") => Command::Deprioritise {
            idx: pargs.free_from_str()?,
        },
        Some("remove" | "rm" | "delete" | "del") => Command::Remove {
            idxs: rest_as_usizes_within_bounds(pargs, n_todos)?,
        },
        Some("do") => Command::Do {
            idxs: rest_as_usizes_within_bounds(pargs, n_todos)?,
        },
        Some("undo") => Command::Undo {
            idxs: rest_as_usizes_within_bounds(pargs, n_dones)?,
        },
        Some("schedule") => Command::Schedule {
            idx: pargs.free_from_str()?,
            date: rest_as_strings(pargs).join(" "),
        },
        Some("unschedule") => Command::Unschedule {
            idxs: rest_as_usizes_within_bounds(pargs, n_todos)?,
        },
        Some("today") => Command::Today {
            idxs: rest_as_usizes_within_bounds(pargs, n_todos)?,
        },
        Some("list" | "ls") => Command::List {
            filters: rest_as_strings(pargs),
        },
        Some("listpriority" | "lsp") => Command::ListPriority {
            filters: rest_as_strings(pargs),
        },
        Some("listdone" | "lsd" | "done") => Command::ListDone {
            filters: rest_as_strings(pargs),
        },
        Some("listprojects" | "prj" | "lsprj" | "projects") => Command::ListProjects,
        Some("listcontexts" | "con" | "lscon" | "contexts") => Command::ListContexts,
        Some("projectview" | "pv") => Command::ProjectView {
            filters: rest_as_strings(pargs),
        },
        Some("contextview" | "cv") => Command::ContextView {
            filters: rest_as_strings(pargs),
        },
        Some("donesummary" | "ds") => Command::DoneSummary {
            days: pargs.value_from_str("--days").unwrap_or(7),
            filters: rest_as_strings(pargs),
        },

        Some("due") => Command::Due {
            n_days: pargs.value_from_str("--days").unwrap_or(7),
            filters: rest_as_strings(pargs),
        },
        Some("nodate" | "nd") => Command::NoDate {
            filters: rest_as_strings(pargs),
        },
        Some("archive") => Command::Archive,
        // CATCH ALL FOR UNRECOGNISED COMMAND
        Some(unrecognised) => {
            println!("Command {:#?} not recognised.", unrecognised);
            println!("{}", USAGE);
            std::process::exit(1);
        }
        _ => {
            println!("{}", USAGE);
            std::process::exit(1);
        }
    };
    debug!("Parsed Command: {:#?}", command);

    Ok((command, autoarchive))
}
