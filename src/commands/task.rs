use crate::{
    api::{ClockifyCLI, TaskRequest},
    utils::{clear_screen, date, parse_duration, read, cursor, datetime}, cfg::SavedTask,
};
use chrono::SecondsFormat;
use clap::{Parser, Subcommand};
use inquire::{DateSelect};

#[derive(Debug, Parser)]
pub struct TaskCommand {
    #[clap(subcommand)]
    command: AddSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum AddSubCommand {
    /// Create a new task
    Add,
    /// Delete a task
    Delete,
    /// List all registered tasks
    List,
    /// Creates a new task from a saved template
    Saved,
}

impl TaskCommand {
    pub async fn run(&self, cli: &mut ClockifyCLI) {
        let api = &cli.api;
        match &self.command {
            AddSubCommand::Add => {
                let workspace = cli.select_workspace().await.unwrap();
                let project = cli.select_project(&workspace).await;
                let tags = cli.select_tags(&workspace).await;
                let description = ClockifyCLI::select_text("Enter a description").await;
                let start = ClockifyCLI::select_time(None).await;
                let end = ClockifyCLI::select_time(start).await;
                let mut request = TaskRequest {
                    description: description.unwrap(),
                    start: date(start.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true),
                    end: date(end.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true),
                    billable: true,
                    project_id: project.unwrap().id,
                    task_id: None,
                    tag_ids: tags.unwrap().iter().map(|tag| tag.id.clone()).collect(),
                    custom_fields: Vec::new(),
                };

                let custom_date = ClockifyCLI::select_bool("Do you want to change the date of the task?");

                if custom_date {
                    let date = DateSelect::new("Select a date:").prompt().unwrap();
                    request.start = datetime(date, start.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true);
                    request.end = datetime(date, end.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true);
                } else {
                    request.start = date(start.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true);
                    request.end = date(end.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true);
                }
                let task = api.new_task(&workspace, &request).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    clear_screen();
                    println!("Failed to create task");
                    println!("Please report this issue!");
                    return;
                }
                let save = ClockifyCLI::select_bool("Do you want to save this task as a template?");
                if save {
                    let alias = ClockifyCLI::select_text("Enter a name for this template").await;
                    let api = &mut cli.api;
                    let mgr = &mut api.manager;
                    let saved = SavedTask {
                        task: request.clone(),
                        name: alias.unwrap(),
                        start: start.unwrap(),
                        end: end.unwrap()
                    };
                    mgr.save_task(saved);
                    mgr.save();
                }
                clear_screen();
                println!("Task created successfully");
                println!("Thank you for using Clockify CLI <3!");
            }
            AddSubCommand::Delete => {
                let workspace = cli.select_workspace().await.unwrap();
                let task = cli.select_task(&workspace).await.unwrap();
                let task = api.delete_task(&workspace, &task).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to delete task");
                    return;
                }
                println!("Task deleted successfully");
            }
            AddSubCommand::List => {
                let workspace = cli.select_workspace().await.unwrap();
                let tasks = api.get_tasks(&workspace).await;
                if tasks.is_none() {
                    println!("No tasks found");
                    return;
                }
                clear_screen();
                println!("Registered tasks:\n");
                for (idx, task) in tasks.unwrap().into_iter().enumerate() {
                    println!(
                        "[{}] {} [{}]",
                        idx + 1,
                        task.description,
                        parse_duration(&task.time.duration[2..])
                    );
                }
            }
            AddSubCommand::Saved => {
                let cfg = &cli.api.manager.config;
                let saved = &cfg.as_ref().unwrap().saved_tasks;
                if saved.is_empty() {
                    println!("No saved templates found");
                    return;
                }
                clear_screen();
                println!("Select a saved template to create:\n");
                for (idx, saved) in saved.iter().enumerate() {
                    println!("[{}] {}", idx + 1, saved.name);
                }
                cursor();
                let idx = read::<usize>();
                if idx.is_none() || idx.unwrap() > saved.len() {
                    println!("Invalid index");
                    return;
                }
                let entry = &saved[idx.unwrap() - 1];
                let mut saved = entry.task.clone();
                let workspace = cli.select_workspace().await.unwrap();
                let name = ClockifyCLI::select_text_opt("Do you want to change the description?", Some(&saved.description)).await;
                saved.description = name.unwrap_or(saved.description);
                let custom_date = ClockifyCLI::select_bool("Do you want to change the date of the task?");

                if custom_date {
                    let date = DateSelect::new("Select a date:").prompt().unwrap();
                    saved.start = datetime(date, entry.start).to_rfc3339_opts(SecondsFormat::Millis, true);
                    saved.end = datetime(date, entry.end).to_rfc3339_opts(SecondsFormat::Millis, true);
                } else {
                    saved.start = date(entry.start).to_rfc3339_opts(SecondsFormat::Millis, true);
                    saved.end = date(entry.end).to_rfc3339_opts(SecondsFormat::Millis, true);
                }
                let task = api.new_task(&workspace, &saved).await;
                clear_screen();
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to create task");
                    println!("Please report this issue!");
                    return;
                }
                println!("Task created successfully");
                println!("Thank you for using Clockify CLI <3!");
            }
        }
    }
}
