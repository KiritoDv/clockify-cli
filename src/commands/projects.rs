use clap::Parser;

use crate::{
    api::ClockifyCLI,
    utils::{clear_screen, parse_duration},
};

/// List all clockify tags which are available to the user
#[derive(Debug, Parser)]
pub struct ProjectsCommand;

impl ProjectsCommand {
    pub async fn run(&self, cli: &ClockifyCLI) {
        let api = &cli.api;
        let workspace = cli.select_workspace().await.unwrap();
        let projects = api.get_projects(&workspace).await;
        if projects.is_none() {
            println!("No projects found");
            return;
        }
        clear_screen();
        println!("Registered projects:\n");
        for (idx, project) in projects.unwrap().into_iter().enumerate() {
            let tracked = parse_duration(&project.duration[2..]);
            println!("[{}] {} [{}]", idx + 1, project.name, tracked);
        }
    }
}
