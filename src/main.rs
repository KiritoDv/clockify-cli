#![feature(fs_try_exists)]
use api::{Clockify, ClockifyCLI};
use cfg::ConfigManager;
use clap::Parser;
use commands::{
    config::ConfigCommand, projects::ProjectsCommand, tags::TagsCommand, task::TaskCommand,
};

pub mod api;
pub mod cfg;
pub mod commands;
pub mod utils;

#[derive(Debug, Parser)]
#[clap(name = "clockify", version)]
pub enum App {
    Config(ConfigCommand),
    /// Manage clockify tasks
    Task(TaskCommand),
    /// List all clockify tags which are available to the user
    Tags(TagsCommand),
    /// List all clockify projects which are available to the user
    Projects(ProjectsCommand),
}

#[tokio::main]
async fn main() {
    let mut mgr = ConfigManager { config: None };
    mgr.load();
    let api = Clockify { manager: mgr };
    let mut cli = ClockifyCLI { api };
    let args = App::parse();
    match args {
        App::Config(config) => config.run(&mut cli).await,
        App::Task(task) => task.run(&mut cli).await,
        App::Tags(tags) => tags.run(&cli).await,
        App::Projects(projects) => projects.run(&cli).await,
    }
}
