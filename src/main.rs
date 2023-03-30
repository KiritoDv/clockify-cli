#![feature(fs_try_exists)]
use std::sync::{Arc, Mutex};

use api::Clockify;
use cfg::ConfigManager;
use clap::Parser;
use lazy_static::lazy_static;
use commands::{projects::ProjectsCommand, tags::TagsCommand, task::TaskCommand, config::ConfigCommand};

pub mod api;
pub mod cfg;
pub mod commands;
pub mod utils;

pub const API: Clockify = Clockify;
lazy_static! {
    pub static ref CONFIG: Arc<Mutex<ConfigManager>> = Arc::new(Mutex::new(ConfigManager {
        config: None,
    }));
}

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
    CONFIG.lock().unwrap().load();
    let args = App::parse();
    match args {
        App::Config(config) => config.run().await,
        App::Task(task) => task.run().await,
        App::Tags(tags) => tags.run().await,
        App::Projects(projects) => projects.run().await,
    }
}
