use crate::{
    utils::clear_screen,
    API, CONFIG,
};
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
    pub async fn run(&self) {
        match &self.command {
            ConfigSubCommand::Login { api_key } => {
                let mut mgr = CONFIG.lock().unwrap();
                let config = mgr.config.as_mut().unwrap();
                config.api_key = api_key.clone();
                drop(mgr);
                let user = API.get_user().await;
                if user.is_none() {
                    println!("Invalid API key");
                    return;
                }
                clear_screen();
                println!("Logged in successfully");
                CONFIG.lock().unwrap().save();
            },
        }
    }
}
