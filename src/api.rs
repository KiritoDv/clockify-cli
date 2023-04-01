use crate::{
    cfg::ConfigManager,
    utils::{clear_screen, cursor, parse_duration, read},
};
use chrono::NaiveTime;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
pub struct ClockifyCLI {
    pub api: Clockify,
}

pub struct Clockify {
    pub manager: ConfigManager,
}
#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub duration: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Workspace {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TaskInterval {
    pub start: String,
    pub end: String,
    pub duration: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    #[serde(rename = "timeInterval")]
    pub time: TaskInterval,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TaskRequest {
    pub start: String,
    pub billable: bool,
    pub description: String,
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "taskId")]
    pub task_id: Option<String>,
    pub end: String,
    #[serde(rename = "tagIds")]
    pub tag_ids: Vec<String>,
    #[serde(rename = "customFields")]
    pub custom_fields: Vec<String>,
}

impl Clockify {
    pub fn gen_auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let api_key = &self.manager.config.as_ref().unwrap().api_key;
        headers.insert("X-Api-Key", api_key.parse().unwrap());
        headers
    }

    pub async fn get_user(&self) -> Option<User> {
        let client = reqwest::Client::new();
        let result = client
            .get("https://api.clockify.me/api/v1/user")
            .headers(self.gen_auth_headers())
            .send()
            .await
            .unwrap()
            .json::<User>()
            .await;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            return None;
        }

        Some(result.unwrap())
    }

    pub async fn get_workspaces(&self) -> Option<Vec<Workspace>> {
        let client = reqwest::Client::new();
        let result = client
            .get("https://api.clockify.me/api/v1/workspaces")
            .headers(self.gen_auth_headers())
            .send()
            .await
            .unwrap()
            .json::<Vec<Workspace>>()
            .await;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            return None;
        }

        Some(result.unwrap())
    }

    pub async fn get_tags(&self, workspace: &Workspace) -> Option<Vec<Tag>> {
        let client = reqwest::Client::new();
        let result = client
            .get(format!(
                "https://api.clockify.me/api/v1/workspaces/{}/tags",
                workspace.id
            ))
            .headers(self.gen_auth_headers())
            .send()
            .await
            .unwrap()
            .json::<Vec<Tag>>()
            .await;

        if result.is_err() {
            print!("Error: {}", result.err().unwrap());
            return None;
        }

        Some(result.unwrap())
    }

    pub async fn get_projects(&self, workspace: &Workspace) -> Option<Vec<Project>> {
        let client = reqwest::Client::new();
        let result = client
            .get(format!("https://api.clockify.me/api/v1/workspaces/{}/projects?name=&archived=false&hydrated=true", workspace.id))
            .headers(self.gen_auth_headers())
            .send().await
            .unwrap()
            .json::<Vec<Project>>().await;

        if result.is_err() {
            print!("Error: {}", result.err().unwrap());
            return None;
        }

        Some(result.unwrap())
    }

    pub async fn get_tasks(&self, workspace: &Workspace) -> Option<Vec<Task>> {
        let user = self.get_user().await.unwrap();
        let client = reqwest::Client::new();
        let result = client
            .get(format!(
                "https://api.clockify.me/api/v1/workspaces/{}/user/{}/time-entries",
                workspace.id, user.id
            ))
            .headers(self.gen_auth_headers())
            .send()
            .await
            .unwrap()
            .json::<Vec<Task>>()
            .await;

        if result.is_err() {
            println!("Error: {}", result.err().unwrap());
            return None;
        }

        Some(result.unwrap())
    }

    pub async fn new_task(&self, workspace: &Workspace, request: &TaskRequest) -> Option<bool> {
        let client = reqwest::Client::new();
        let result = client
            .post(format!(
                "https://global.api.clockify.me/workspaces/{}/timeEntries/full",
                workspace.id
            ))
            .headers(self.gen_auth_headers())
            .json::<TaskRequest>(&request)
            .send()
            .await
            .unwrap();

        Some(result.status().is_success())
    }

    pub async fn delete_task(&self, workspace: &Workspace, task: &Task) -> Option<bool> {
        let client = reqwest::Client::new();
        let result = client
            .delete(format!(
                "https://api.clockify.me/api/v1/workspaces/{}/time-entries/{}",
                workspace.id, task.id
            ))
            .headers(self.gen_auth_headers())
            .send()
            .await
            .unwrap();

        Some(result.status().is_success())
    }
}

impl ClockifyCLI {
    pub async fn select_workspace(&self) -> Option<Workspace> {
        let workspaces = self.api.get_workspaces().await;
        if workspaces.is_none() {
            println!("No Workspaces found");
            return None;
        }

        let length = workspaces.as_ref().unwrap().len();

        loop {
            clear_screen();
            println!("Select a workspace:\n");
            for (idx, workspace) in workspaces.as_ref().unwrap().iter().enumerate() {
                println!("[{}] {}", idx + 1, workspace.name);
            }
            cursor();
            let data = read::<usize>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let idx = data.unwrap();
            if idx > length {
                println!("Invalid input");
                continue;
            }
            return Some(workspaces.as_ref().unwrap()[idx - 1].clone());
        }
    }

    pub async fn select_project(&self, workspace: &Workspace) -> Option<Project> {
        let projects = self.api.get_projects(workspace).await;
        if projects.is_none() {
            println!("No projects found");
            return None;
        }

        let length = projects.as_ref().unwrap().len();

        loop {
            clear_screen();
            println!("Select a project:\n");
            for (idx, project) in projects.as_ref().unwrap().iter().enumerate() {
                let tracked = parse_duration(&project.duration[2..]);
                println!("[{}] {} [{}]", idx + 1, project.name, tracked);
            }
            cursor();
            let data = read::<usize>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let idx = data.unwrap();
            if idx > length {
                println!("Invalid input");
                continue;
            }
            return Some(projects.as_ref().unwrap()[idx - 1].clone());
        }
    }

    pub async fn select_task(&self, workspace: &Workspace) -> Option<Task> {
        let entries = self.api.get_tasks(workspace).await;
        if entries.is_none() {
            println!("No projects found");
            return None;
        }

        let length = entries.as_ref().unwrap().len();

        loop {
            clear_screen();
            println!("Select a task:\n");
            for (idx, entry) in entries.as_ref().unwrap().iter().enumerate() {
                println!(
                    "[{}] {} [{}]",
                    idx + 1,
                    entry.description,
                    parse_duration(&entry.time.duration[2..])
                );
            }
            cursor();
            let data = read::<usize>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let idx = data.unwrap();
            if idx > length {
                println!("Invalid input");
                continue;
            }
            return Some(entries.as_ref().unwrap()[idx - 1].clone());
        }
    }

    pub async fn select_tags(&self, workspace: &Workspace) -> Option<Vec<Tag>> {
        let mut selected_tags: Vec<Tag> = Vec::new();
        let tags = self.api.get_tags(workspace).await;
        if tags.is_none() {
            println!("No tags found");
            return None;
        }

        let length = tags.as_ref().unwrap().len();

        loop {
            clear_screen();
            println!("Select a tag:\n");
            for (idx, tag) in tags.as_ref().unwrap().iter().enumerate() {
                let selected = if selected_tags.contains(tag) {
                    "X"
                } else {
                    " "
                };
                println!("[{}] {} [{}]", idx + 1, tag.name, selected);
            }
            println!("[0] Continue");
            cursor();
            let data = read::<usize>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let idx = data.unwrap();
            if idx == 0 {
                return Some(selected_tags);
            }
            if idx > length {
                println!("Invalid tag");
                continue;
            }
            if selected_tags.contains(&tags.as_ref().unwrap()[idx - 1]) {
                selected_tags.remove(idx - 1);
                continue;
            }
            selected_tags.push(tags.as_ref().unwrap()[idx - 1].clone());
        }
    }

    pub async fn select_text_opt(text: &str, default: Option<&str>) -> Option<String> {
        let mut selected_text = default.unwrap_or("").to_string();
        loop {
            clear_screen();
            if selected_text.is_empty() {
                println!("{}:\n", text);
            } else {
                println!("{} [{}]:\n", text, selected_text);
            }
            println!("[0] Continue");
            cursor();
            let data = read::<String>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let description = data.unwrap();
            if description == "0" {
                return Some(selected_text);
            }
            selected_text = description;
        }
    }

    pub async fn select_text(text: &str) -> Option<String> {
        return Self::select_text_opt(text, None).await;
    }

    pub async fn select_time(start: Option<NaiveTime>) -> Option<NaiveTime> {
        let mut selected_time: Option<NaiveTime> = None;
        loop {
            clear_screen();
            if selected_time.is_none() {
                println!(
                    "Please enter the {} time (HH:MM):",
                    if start.is_some() { "end" } else { "start" }
                );
            } else {
                println!(
                    "Please enter the {} time ({}):",
                    if start.is_some() { "end" } else { "start" },
                    selected_time.unwrap()
                );
            }
            println!("[0] Continue");
            cursor();
            let data = read::<String>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let description = data.unwrap();
            if description == "0" {
                return selected_time;
            }
            let time = NaiveTime::parse_from_str(&description, "%H:%M");
            if time.is_err() {
                println!("Invalid time");
                continue;
            }
            if start.is_some() && start.unwrap() > time.unwrap() {
                println!("Start time cannot be after end time");
                continue;
            }
            selected_time = Some(time.unwrap());
        }
    }
    pub fn select_bool(text: &str) -> bool {
        let mut status: bool = false;
        loop {
            clear_screen();
            println!("{} (y/n): {}\n", text, if status { "Yes" } else { "No" });
            println!("[0] Continue");
            cursor();
            let data = read::<String>();
            if data.is_none() {
                println!("Invalid input");
                continue;
            }
            let description = data.unwrap();
            if description == "0" {
                return status;
            }
            if description == "y" {
                status = true;
                continue;
            }
            if description == "n" {
                status = false;
                continue;
            }
        }
    }
}
