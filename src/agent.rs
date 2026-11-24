use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    #[serde(rename = "nom")]
    pub name: String,
    #[serde(rename = "image")]
    pub image_path: String,
    #[serde(default)]
    pub role: Option<Role>,
    #[serde(default)]
    pub custom: bool,
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

    pub fn label(self) -> &'static str {
        match self {
            Role::Duelist => "Duelliste",
            Role::Initiator => "Initiateur",
            Role::Controller => "Contrôleur",
            Role::Sentinel => "Sentinelle",
            Role::Unknown => "Non défini",
        }
    }

    pub fn for_agent(name: &str) -> Role {
        match name {
            "Phoenix" | "Jett" | "Raze" | "Reyna" | "Yoru" | "Neon" | "Iso" | "Waylay" | "Hush" => Role::Duelist,
            "Sova" | "Breach" | "Skye" | "KAY/O" | "Fade" | "Gekko" | "Tejo" => Role::Initiator,
            "Brimstone" | "Omen" | "Viper" | "Astra" | "Harbor" | "Clove" | "Miks" => Role::Controller,
            "Sage" | "Killjoy" | "Cypher" | "Chamber" | "Deadlock" | "Vyse" => Role::Sentinel,
            _ => Role::Unknown,
        }
    }
}
