extern crate anyhow;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use clap::{Parser, Subcommand};

mod colour;
mod modify;
mod todo;
mod utility;
mod view;

#[derive(Debug, Parser)]
#[command(name = "t", about = "kinda like todo.sh")]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    auto_archive: bool,
}

#[derive(Subcommand, Debug, Clone)]
#[command(rename_all = "camel")]
enum Command {
    /// Add a task
    #[command(visible_aliases = &["a"])]
    Add { text: String },
    /// Add a task and complete immediately
    Addx { text: String },
    /// Add a task and prioritise as 'A'
    Adda { text: String },
    /// Add a task and schedule today
    Addt { text: String },
    /// Append text to a task
    #[command(visible_aliases = &["app"])]
    Append { idx: usize, text: String },
    /// Prepend text to a task
    #[command(visible_aliases = &["pre"])]
    Prepend { idx: usize, text: String },
    /// Prioritise a task
    #[command(visible_aliases = &["pri", "p"])]
    Prioritise { idx: usize, priority: String },
    /// Deprioritise a task
    #[command(visible_aliases = &["depri", "dp"])]
    Deprioritise { idx: usize },
    /// Remove a task
    #[command(visible_aliases = &["rm"])]
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
    #[command(visible_aliases = &["ls"])]
    List { filters: Vec<String> },
    /// View tasks with a priority
    #[command(visible_aliases = &["lsp"])]
    ListPriority { filters: Vec<String> },
    /// View done tasks
    ListDone { filters: Vec<String> },
    /// Projects
    #[command(visible_aliases = &["proj"])]
    ListProjects,
    /// Without a project
    #[command(visible_aliases = &["noproj", "projectless"])]
    NoProjects,
    /// Tags
    #[command(visible_aliases = &["tag"])]
    ListTags,
    /// Without a tag
    #[command(visible_aliases = &["notag", "tagless"])]
    NoTags,
    /// Open link in task
    #[command(visible_aliases = &["open", "url", "urls"])]
    Link { indices: Vec<usize> },
    /// View tasks grouped by project
    #[command(visible_aliases = &["pv"])]
    ProjectView { filters: Vec<String> },
    /// View tasks grouped by context
    #[command(visible_aliases = &["tv"])]
    TagView { filters: Vec<String> },
    /// View done tasks, by date, for last N days
    #[command(visible_aliases = &["ds"])]
    DoneSummary { days: i64, filters: Vec<String> },
    /// View scheduled tasks
    Due { n_days: usize, filters: Vec<String> },
    /// View unscheduled tasks
    NoDate { filters: Vec<String> },
    /// Move done tasks into DONEFILE
    Archive,
}

type Result<T> = ::std::result::Result<T, Box<dyn (::std::error::Error)>>;

fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let t_dont_autoarchive_env =
        std::env::var("T_DONT_AUTOARCHIVE").unwrap_or_else(|_| "false".to_string());

    let opts = Cli::parse();

    let mut autoarchive =
        opts.auto_archive || t_dont_autoarchive_env.is_empty() || t_dont_autoarchive_env == "false";

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

    // let (command, mut autoarchive) = parse_args(num_todos_at_start, num_done_at_start)?;
    debug!("Autoarchiving? {}", autoarchive);

    let result = match opts.command {
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
        Command::NoProjects => view::no_projects(todos.iter()),
        Command::ListTags => view::tags(todos.iter()),
        Command::NoTags => view::no_tags(todos.iter()),
        Command::ProjectView { filters } => view::grouped_by_project(todos.iter(), &filters),
        Command::TagView { filters } => view::grouped_by_tag(todos.iter(), &filters),
        // ========== Date-based views
        Command::Due { n_days, filters } => view::due(todos.iter(), n_days, &filters),
        Command::NoDate { filters } => view::no_date(todos.iter(), &filters),
        // ========== Utility
        Command::Link { indices } => view::open_link(&todos, &indices),
        Command::Archive => {
            autoarchive = false;
            modify::archive(&mut todos, &mut dones)
        }
    };

    if let Err(err) = result {
        println!("{}", err);
        std::process::exit(1);
    }

    if autoarchive {
        if let Err(err) = modify::archive(&mut todos, &mut dones) {
            println!("{}", err);
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
