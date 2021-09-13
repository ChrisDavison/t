extern crate anyhow;

mod modify;
mod todo;
mod utility;
mod view;

enum Command {
    /// Add a task
    Add { text: Vec<String> },
    /// Append text to a task
    Append { idx: usize, text: Vec<String> },
    /// Prepend text to a task
    Prepend { idx: usize, text: Vec<String> },
    /// Remove a task
    Remove { idxs: Vec<usize> },
    /// Move task to DONEFILE
    Do { idxs: Vec<usize> },
    /// Move task from DONEFILE to TODOFILE
    Undo { idxs: Vec<usize> },
    /// Schedule a task
    Schedule { idxs: Vec<usize>, date: String },
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
    /// View done tasks, by date, for last N days
    DoneSummary { n_days: usize, filters: Vec<String> },
    /// View scheduled tasks
    Due { n_days: usize, filters: Vec<String> },
    /// View unscheduled tasks
    NoDate { filters: Vec<String> },
    /// Move done tasks into DONEFILE
    Archive,
}

#[allow(dead_code)]
const USAGE: &str = "usage: t <COMMAND> [ARGS]...


Commands:
    add TEXT...              [a] Add a task
    append IDX TEXT...       [app] Append TEXT... to task
    prepend IDX TEXT...      [pre] Prepend TEXT... to task
    remove IDX               [rm|del] Remove task
    do IDX                   Move task to DONE
    undo IDX                 Move task from DONE to TODO

    schedule IDX DATE        Schedule task
    unschedule IDX           Remove due date from task
    today IDX                Schedule task for today

    list [FILTER]...         [ls] View tasks
    listdone [FILTER]...     [lsd|done] View done tasks
    listpriority [FILTER]... [lsp] View tasks with a priority
    due [FILTER]...          View scheduled tasks
    nodate [FILTER]...       [nd] view unscheduled tasks

    donesummary [FILTER]...  [ds] view completed tasks in last 7 days

    archive                  move done tasks to archive
    help                     View this message

Note:
    Files are $TODOFILE and $DONEFILE, both following todo.txt syntax

    FILTERS: Any words with '-' prefix will be treated as MUST NOT MATCH. Else,
    all the rest must match.

    IDX refers to the number of the task you wish to modify.
    Words in square brackets are aliases for commands";

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
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

    let (command, mut autoarchive) = parse_pico()?;
    let result = match command {
        // ========== Modification
        Command::Add { text } => modify::add(&text, &mut todos),
        Command::Append { idx, text } => modify::append(idx, &mut todos, &text),
        Command::Prepend { idx, text } => modify::prepend(idx, &mut todos, &text),
        Command::Remove { mut idxs } => modify::remove(&mut idxs, &mut todos),
        Command::Do { mut idxs } => modify::do_task(&mut idxs, &mut todos),
        Command::Undo { mut idxs } => modify::undo(&mut idxs, &mut todos, &mut dones),
        // ========== SCHEDULING
        Command::Schedule { mut idxs, date } => do_to_each::schedule(&mut idxs, &mut todos, &date),
        Command::Unschedule { mut idxs } => do_to_each::unschedule(&mut idxs, &mut todos),
        Command::Today { mut idxs } => do_to_each::today(&mut idxs, &mut todos),
        // ========== Filtered views
        Command::List { filters } => view::list(&todos, &filters),
        Command::ListPriority { filters } => view::list_priority(&todos, &filters),
        Command::ListDone { filters } => view::done(&dones, &filters),
        Command::DoneSummary { n_days, filters } => view::done_summary(&dones, n_days, &filters),
        // ========== Date-based views
        Command::Due { n_days, filters } => view::due(&todos, n_days, &filters),
        Command::NoDate { filters } => view::no_date(&todos, &filters),
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
    utility::save_to_file(&todos, std::env::var("TODOFILE")?)?;
    utility::save_to_file(&dones, std::env::var("DONEFILE")?)?;

    if num_todos_at_start != 0 && todos.is_empty() {
        println!("TODOFILE is now empty");
    }
    if num_done_at_start != 0 && dones.is_empty() {
        println!("DONEFILE is now empty");
    }
    Ok(())
}

// Return the command to execute, and whether to auto-archive
fn parse_pico() -> Result<(Command, bool)> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    let t_dont_autoarchive_env = std::env::var("T_DONT_AUTOARCHIVE").unwrap_or("false".to_string());
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

    let rest_as_usizes = |pargs: pico_args::Arguments| {
        rest_as_strings(pargs)
            .iter()
            .map(|x| x.parse().expect("Failed to parse indice"))
            .collect()
    };

    let command = match pargs.subcommand()?.as_deref() {
        Some("help" | "h") => {
            println!("{}", USAGE);
            std::process::exit(0);
        }
        Some("add" | "a") => Command::Add {
            text: rest_as_strings(pargs),
        },
        Some("append" | "app") => Command::Append {
            idx: pargs.free_from_str()?,
            text: rest_as_strings(pargs),
        },
        Some("prepend" | "pre") => Command::Prepend {
            idx: pargs.free_from_str()?,
            text: rest_as_strings(pargs),
        },
        Some("remove" | "rm" | "delete" | "del") => Command::Remove {
            idxs: rest_as_usizes(pargs),
        },
        Some("do") => Command::Do {
            idxs: rest_as_usizes(pargs),
        },
        Some("undo") => Command::Undo {
            idxs: rest_as_usizes(pargs),
        },
        Some("schedule") => Command::Schedule {
            date: pargs.free_from_str()?,
            idxs: rest_as_usizes(pargs),
        },
        Some("unschedule") => Command::Unschedule {
            idxs: rest_as_usizes(pargs),
        },
        Some("today") => Command::Today {
            idxs: rest_as_usizes(pargs),
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
        Some("donesummary" | "ds") => Command::DoneSummary {
            n_days: pargs.opt_free_from_str()?.unwrap_or(7),
            filters: rest_as_strings(pargs),
        },

        Some("due") => Command::Due {
            n_days: pargs.opt_free_from_str()?.unwrap_or(7),
            filters: rest_as_strings(pargs),
        },
        Some("nodate" | "nd") => Command::NoDate {
            filters: rest_as_strings(pargs),
        },
        Some("archive") => Command::Archive,
        // CATCH ALL FOR UNRECOGNISED COMMAND
        unrecognised => {
            println!("Command {:#?} not recognised.", unrecognised);
            println!("{}", USAGE);
            std::process::exit(1);
        }
    };

    Ok((command, autoarchive))
}

mod do_to_each {
    use super::*;
    pub fn unschedule(args: &mut Vec<usize>, todos: &mut Vec<todo::Todo>) -> Result<()> {
        for i in utility::parse_reversed_indices(args)? {
            if i >= todos.len() {
                continue;
            }
            todos[i].due_date = None;
            todos[i].unschedule();
            utility::notify("UNSCHEDULED", i, &todos[i].task);
        }
        Ok(())
    }

    pub fn today(args: &mut Vec<usize>, todos: &mut Vec<todo::Todo>) -> Result<()> {
        let t_str = utility::get_formatted_date();
        for i in utility::parse_reversed_indices(args)? {
            todos[i].due_date = Some(t_str.clone());
            todos[i].schedule_today();
            utility::notify("TODAY", i, &todos[i].task);
        }
        Ok(())
    }

    pub fn schedule(args: &mut Vec<usize>, todos: &mut Vec<todo::Todo>, date: &str) -> Result<()> {
        for i in utility::parse_reversed_indices(args)? {
            todos[i].due_date = Some(date.to_string());
            todos[i].schedule(date);
            utility::notify("SCHEDULED", i, &todos[i].task);
        }
        Ok(())
    }
}
