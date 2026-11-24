use crate::agent::Agent;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub excluded_agents: Vec<String>,
    pub excluded_roles: Vec<String>,
    pub selected_agents: Vec<String>,
    pub roles_visible: bool,
    pub secret_mode: bool,
    pub secret_target: String,
}

fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("Valoroulette").join("settings.json"))
}

pub fn load_settings() -> Settings {
    let Some(path) = settings_path() else { return Settings::default() };
    fs::read_to_string(path)
        .ok()
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &Settings) {
    let Some(path) = settings_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(raw) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, raw);
    }
}

pub fn load_agents() -> Vec<Agent> {
    let raw = include_str!("../data/agents.json");
    let mut agents: Vec<Agent> = serde_json::from_str(raw).unwrap_or_default();
    for agent in &mut agents {
        agent.role = Some(crate::agent::Role::for_agent(&agent.name));
    }
    agents
}
