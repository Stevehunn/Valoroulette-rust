use crate::agent::Agent;
use crate::i18n::Language;
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
    #[serde(default)]
    pub language: Language,
}

fn settings_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("Valoroulette").join("settings.json"))
}

fn agents_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("Valoroulette").join("agents.json"))
}

pub fn load_settings() -> Settings {
    let Some(path) = settings_path() else {
        return Settings::default();
    };
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
    if let Some(path) = agents_path() {
        if let Ok(raw) = fs::read_to_string(&path) {
            if let Ok(mut agents) = serde_json::from_str::<Vec<Agent>>(&raw) {
                for agent in &mut agents {
                    if agent.role.is_none() {
                        agent.role = Some(crate::agent::Role::for_agent(&agent.name));
                    }
                }
                return agents;
            }
        }
    }
    let raw = include_str!("../data/agents.json");
    let mut agents: Vec<Agent> = serde_json::from_str(raw).unwrap_or_default();
    for agent in &mut agents {
        agent.role = Some(crate::agent::Role::for_agent(&agent.name));
    }
    agents
}

pub fn save_agents(agents: &[Agent]) {
    let Some(path) = agents_path() else { return };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(raw) = serde_json::to_string_pretty(agents) {
        let _ = fs::write(path, raw);
    }
}