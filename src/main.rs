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
    // ------------------------------------------------------------
    //                         Add new tasks
    // ------------------------------------------------------------
    /// Add a task
    #[command(visible_aliases = &["a", "new"])]
    Add { text: String },
    /// Add a task and complete immediately
    Addx { text: String },
    /// Add a task and prioritise as 'A'
    Adda { text: String },
    /// Add a task and schedule today
    Addt { text: String },

    // ------------------------------------------------------------
    //                     Modify existing tasks
    // ------------------------------------------------------------
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
    #[command(visible_aliases = &["dp"])]
    Deprioritise { idx: usize },

    // ------------------------------------------------------------
    //                 Modify completion / existance
    // ------------------------------------------------------------
    /// Remove a task
    #[command(visible_aliases = &["rm", "del"])]
    Remove { idxs: Vec<usize> },
    /// Move task to DONEFILE
    Do { idxs: Vec<usize> },
    /// Move task from DONEFILE to TODOFILE
    Undo { idxs: Vec<usize> },
    /// Move done tasks into DONEFILE
    Archive,

    // ------------------------------------------------------------
    //                          Scheduling
    // ------------------------------------------------------------
    /// Schedule a task
    #[command(visible_aliases = &["s"])]
    Schedule { idx: usize, date: String },
    /// Remove due date from task
    Unschedule { idxs: Vec<usize> },
    /// Schedule task today
    Today { idxs: Vec<usize> },

    // ------------------------------------------------------------
    //                             Views
    // ------------------------------------------------------------
    /// View tasks
    #[command(visible_aliases = &["ls"])]
    List { filters: Vec<String> },
    /// View tasks with a priority
    #[command(visible_aliases = &["lsp"])]
    ListPriority { filters: Vec<String> },
    /// View done tasks
    #[command(visible_aliases = &["lsd"])]
    ListDone { filters: Vec<String> },
    /// View scheduled tasks
    Due { n_days: usize, filters: Vec<String> },
    /// View unscheduled tasks
    NoDate { filters: Vec<String> },
    /// View done tasks, by date, for last N days
    #[command(visible_aliases = &["ds"])]
    DoneSummary { days: i64, filters: Vec<String> },

    // ------------------------------------------------------------
    //                       Views - Projects
    // ------------------------------------------------------------
    /// Projects
    #[command(visible_aliases = &["proj"])]
    Projects,
    /// Without a project
    #[command(visible_aliases = &["noproj"])]
    Projectless,
    /// View tasks grouped by project
    #[command(visible_aliases = &["pv"])]
    ProjectView { filters: Vec<String> },

    // ------------------------------------------------------------
    //                         Views - Tags
    // ------------------------------------------------------------
    /// Tags
    #[command(visible_aliases = &["tag"])]
    Tags,
    /// Without a tag
    #[command(visible_aliases = &["notag"])]
    Tagless,
    /// View tasks grouped by context
    #[command(visible_aliases = &["tv"])]
    TagView { filters: Vec<String> },

    // ------------------------------------------------------------
    //                            Utility
    // ------------------------------------------------------------
    /// Open link in task
    #[command(visible_aliases = &["open", "url", "urls"])]
    Link { indices: Vec<usize> },
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
        // ------------------------------------------------------------
        //                         Add new tasks
        // ------------------------------------------------------------
        Command::Add { text } => modify::add(&text, &mut todos),
        Command::Addx { text } => modify::addx(&text, &mut todos),
        Command::Adda { text } => modify::adda(&text, &mut todos),
        Command::Addt { text } => modify::addt(&text, &mut todos),

        // ------------------------------------------------------------
        //                     Modify existing tasks
        // ------------------------------------------------------------
        Command::Append { idx, text } => modify::append(idx, &mut todos, &text),
        Command::Prepend { idx, text } => modify::prepend(idx, &mut todos, &text),
        Command::Prioritise { idx, priority } => {
            modify::prioritise(idx, &mut todos, Some(priority))
        }
        Command::Deprioritise { idx } => modify::prioritise(idx, &mut todos, None),

        // ------------------------------------------------------------
        //                 Modify completion / existance
        // ------------------------------------------------------------
        Command::Remove { idxs } => modify::remove(&idxs, &mut todos),
        Command::Do { idxs } => modify::do_task(&idxs, &mut todos),
        Command::Undo { idxs } => modify::undo(&idxs, &mut todos, &mut dones),
        Command::Archive => {
            autoarchive = false;
            modify::archive(&mut todos, &mut dones)
        }

        // ------------------------------------------------------------
        //                          Scheduling
        // ------------------------------------------------------------
        Command::Schedule { idx, date } => modify::schedule(idx, &mut todos, &date),
        Command::Unschedule { idxs } => modify::unschedule_each(&idxs, &mut todos),
        Command::Today { idxs } => modify::schedule_each_today(&idxs, &mut todos),

        // ------------------------------------------------------------
        //                             Views
        // ------------------------------------------------------------
        Command::List { filters } => view::list(todos.iter(), &filters),
        Command::ListPriority { filters } => view::list_priority(todos.iter(), &filters),
        Command::ListDone { filters } => view::done(dones.iter(), &filters),
        Command::Due { n_days, filters } => view::due(todos.iter(), n_days, &filters),
        Command::NoDate { filters } => view::no_date(todos.iter(), &filters),
        Command::DoneSummary { days, filters } => view::done_summary(dones.iter(), &filters, days),

        // ------------------------------------------------------------
        //                       Views - Projects
        // ------------------------------------------------------------
        Command::Projects => view::projects(todos.iter()),
        Command::Projectless => view::no_projects(todos.iter()),
        Command::ProjectView { filters } => view::grouped_by_project(todos.iter(), &filters),

        // ------------------------------------------------------------
        //                         Views - Tags
        // ------------------------------------------------------------
        Command::Tags => view::tags(todos.iter()),
        Command::Tagless => view::no_tags(todos.iter()),
        Command::TagView { filters } => view::grouped_by_tag(todos.iter(), &filters),

        // ------------------------------------------------------------
        //                            Utility
        // ------------------------------------------------------------
        Command::Link { indices } => view::open_link(&todos, &indices),
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
