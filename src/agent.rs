use crate::i18n::Language;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    #[serde(default)]
    pub role: Option<Role>,
    #[serde(default)]
    pub custom: bool,
    #[serde(default)]
    pub image_path: String,
    #[serde(default)]
    pub custom_role: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Duelist,
    Initiator,
    Controller,
    Sentinel,
    Unknown,
}

impl Role {
    pub const ALL: [Role; 4] = [
        Role::Duelist,
        Role::Initiator,
        Role::Controller,
        Role::Sentinel,
    ];

    /// Returns the technical key used for saving exclusions (independent of language).
    pub fn as_key(self) -> &'static str {
        match self {
            Role::Duelist => "duelist",
            Role::Initiator => "initiator",
            Role::Controller => "controller",
            Role::Sentinel => "sentinel",
            Role::Unknown => "unknown",
        }
    }

    /// Returns the translated label for the UI.
    pub fn label(self, lang: Language) -> &'static str {
        let t = lang.translations();
        match self {
            Role::Duelist => t.role_duelist,
            Role::Initiator => t.role_initiator,
            Role::Controller => t.role_controller,
            Role::Sentinel => t.role_sentinel,
            Role::Unknown => t.role_unknown,
        }
    }

    pub fn for_agent(name: &str) -> Role {
        match name {
            "Phoenix" | "Jett" | "Raze" | "Reyna" | "Yoru" | "Neon" | "Iso" | "Waylay"
            | "Hush" => Role::Duelist,
            "Sova" | "Breach" | "Skye" | "KAY/O" | "Fade" | "Gekko" | "Tejo" => {
                Role::Initiator
            }
            "Brimstone" | "Omen" | "Viper" | "Astra" | "Harbor" | "Clove" | "Miks" => {
                Role::Controller
            }
            "Sage" | "Killjoy" | "Cypher" | "Chamber" | "Deadlock" | "Vyse" | "Veto" => {
                Role::Sentinel
            }
            _ => Role::Unknown,
        }
    }
}