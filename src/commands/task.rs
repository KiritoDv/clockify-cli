use crate::{
    api::{ClockifyCLI, TaskRequest},
    utils::{clear_screen, date, parse_duration, read, cursor}, cfg::SavedTask,
};
use chrono::SecondsFormat;
use clap::{Parser, Subcommand};

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
                let request = TaskRequest {
                    description: description.unwrap(),
                    start: date(start.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true),
                    end: date(end.unwrap()).to_rfc3339_opts(SecondsFormat::Millis, true),
                    billable: true,
                    project_id: project.unwrap().id,
                    task_id: None,
                    tag_ids: tags.unwrap().iter().map(|tag| tag.id.clone()).collect(),
                    custom_fields: Vec::new(),
                };
                let task = api.new_task(&workspace, &request).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to create task");
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
                println!("Task created successfully");
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
                saved.start = date(entry.start).to_rfc3339_opts(SecondsFormat::Millis, true);
                saved.end = date(entry.end).to_rfc3339_opts(SecondsFormat::Millis, true);
                let workspace = cli.select_workspace().await.unwrap();
                let name = ClockifyCLI::select_text_opt("Do you want to change the description?", Some(&saved.description)).await;
                saved.description = name.unwrap_or(saved.description);
                let task = api.new_task(&workspace, &saved).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to create task");
                    return;
                }
                println!("Task created successfully");
            }
        }
    }
}
