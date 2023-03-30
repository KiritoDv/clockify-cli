use clap::Parser;

use crate::{api::ClockifyCLI, utils::clear_screen, API};

/// List all clockify tags which are available to the user
#[derive(Debug, Parser)]
pub struct TagsCommand;

impl TagsCommand {
    pub async fn run(&self) {
        let workspace = ClockifyCLI::select_workspace().await.unwrap();
        let tags = API.get_tags(&workspace).await;
        if tags.is_none() {
            println!("No tags found");
            return;
        }
        clear_screen();
        println!("Clockify tags:\n");
        for (idx, tag) in tags.unwrap().into_iter().enumerate() {
            println!(" [{}] {}", idx + 1, tag.name);
        }
    }
}
