use std::path::PathBuf;

use anyhow::{anyhow, Ok};
use cli::CommandLineArgs;
use structopt::StructOpt;

extern crate structopt;
mod cli;
mod tasks;

use cli::Action::*;
use tasks::Task;
fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        jounal_file,
    } = CommandLineArgs::from_args();

    let journal_file = jounal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".my-todo");
        path
    })
}
