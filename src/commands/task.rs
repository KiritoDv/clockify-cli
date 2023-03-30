use crate::{
    api::{ClockifyCLI, TaskRequest},
    utils::{clear_screen, date, parse_duration},
    API,
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
    /// Create a new time entry
    Add,
    /// Delete a time entry
    Delete,
    /// List all registered time entries
    List,
}

impl TaskCommand {
    pub async fn run(&self) {
        match &self.command {
            AddSubCommand::Add => {
                let workspace = ClockifyCLI::select_workspace().await.unwrap();
                let project = ClockifyCLI::select_project(&workspace).await;
                let tags = ClockifyCLI::select_tags(&workspace).await;
                let description = ClockifyCLI::select_description().await;
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
                let task = API.new_task(&workspace, request).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to create task");
                    return;
                }
                println!("Task created successfully");
            },
            AddSubCommand::Delete => {
                let workspace = ClockifyCLI::select_workspace().await.unwrap();
                let task = ClockifyCLI::select_task(&workspace).await.unwrap();
                let task = API.delete_task(&workspace, &task).await;
                if task.is_none() || !task.as_ref().unwrap() {
                    println!("Failed to delete task");
                    return;
                }
                println!("Task deleted successfully");
            }
            AddSubCommand::List => {
                let workspace = ClockifyCLI::select_workspace().await.unwrap();
                let tasks = API.get_tasks(&workspace).await;
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
        }
    }
}
