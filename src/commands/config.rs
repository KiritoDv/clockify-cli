use crate::{api::{ClockifyCLI}, utils::clear_screen};
use clap::{Parser, Subcommand};

/// Configure the authentication token
#[derive(Debug, Parser)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    command: ConfigSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubCommand {
    /// Sets the API key
    Login {
        /// The API key to use
        api_key: String,
    },
}

impl ConfigCommand {
    pub async fn run(&self, cli: &mut ClockifyCLI) {
        let api = &mut cli.api;
        let mgr = &mut api.manager.config;
        match &self.command {
            ConfigSubCommand::Login { api_key } => {
                mgr.as_mut().unwrap().api_key = api_key.clone();
                let user = api.get_user().await;
                if user.is_none() {
                    println!("Invalid API key");
                    return;
                }
                clear_screen();
                println!("Logged in successfully");
                api.manager.save();
            }
        }
    }
}
