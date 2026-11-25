use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Fr,
    En,
    Es,
}

impl Default for Language {
    fn default() -> Self {
        Language::Fr
    }
}

impl Language {
    pub const ALL: [Language; 3] = [Language::Fr, Language::En, Language::Es];

    pub fn label(&self) -> &'static str {
        match self {
            Language::Fr => "Français",
            Language::En => "English",
            Language::Es => "Español",
        }
    }

    pub fn translations(&self) -> &'static Translations {
        match self {
            Language::Fr => &FR,
            Language::En => &EN,
            Language::Es => &ES,
        }
    }
}

pub struct Translations {
    pub mode_secret: &'static str,
    pub secret_hint: &'static str,
    pub target: &'static str,
    pub add_agent: &'static str,
    pub agent_name: &'static str,
    pub role: &'static str,
    pub custom_role: &'static str,
    pub choose_image: &'static str,
    pub add_btn: &'static str,
    pub filters: &'static str,
    pub excluded_agents: &'static str,
    pub no_excluded_agent: &'static str,
    pub restore_all: &'static str,
    pub my_agents: &'static str,
    pub delete_btn: &'static str,
    pub exclude_btn: &'static str,
    pub spin_btn: &'static str,
    pub spin_secret_btn: &'static str,
    pub no_agent_available: &'static str,
    pub language: &'static str,
    
    // Update banner strings
    pub update_available: &'static str,
    pub download: &'static str,
    pub downloading: &'static str,
    pub download_success: &'static str,
    pub download_error: &'static str,

    // Roles
    pub role_duelist: &'static str,
    pub role_initiator: &'static str,
    pub role_controller: &'static str,
    pub role_sentinel: &'static str,
    pub role_unknown: &'static str,
}

static FR: Translations = Translations {
    mode_secret: "Mode secret",
    secret_hint: "Ctrl+clic ou Alt+clic sur LANCER",
    target: "Cible :",
    add_agent: "Ajouter un agent",
    agent_name: "Nom de l'agent",
    role: "Rôle :",
    custom_role: "Rôle personnalisé (optionnel)",
    choose_image: "Choisir une image",
    add_btn: "AJOUTER",
    filters: "Filtres par rôle",
    excluded_agents: "Agents exclus",
    no_excluded_agent: "Aucun agent exclu",
    restore_all: "↺ RESTAURER TOUT",
    my_agents: "Mes agents",
    delete_btn: "Supprimer",
    exclude_btn: "EXCLURE",
    spin_btn: "LANCER",
    spin_secret_btn: "LANCER (mode secret)",
    no_agent_available: "Aucun agent disponible",
    language: "Langue",
    
    update_available: "Mise à jour disponible",
    download: "Télécharger",
    downloading: "Téléchargement en cours...",
    download_success: "Téléchargé dans le dossier Téléchargements !",
    download_error: "Échec du téléchargement",
    
    role_duelist: "Duelliste",
    role_initiator: "Initiateur",
    role_controller: "Contrôleur",
    role_sentinel: "Sentinelle",
    role_unknown: "Non défini",
};

static EN: Translations = Translations {
    mode_secret: "Secret mode",
    secret_hint: "Ctrl+click or Alt+click on SPIN",
    target: "Target:",
    add_agent: "Add an agent",
    agent_name: "Agent name",
    role: "Role:",
    custom_role: "Custom role (optional)",
    choose_image: "Choose an image",
    add_btn: "ADD",
    filters: "Role filters",
    excluded_agents: "Excluded agents",
    no_excluded_agent: "No excluded agent",
    restore_all: "↺ RESTORE ALL",
    my_agents: "My agents",
    delete_btn: "Delete",
    exclude_btn: "EXCLUDE",
    spin_btn: "SPIN",
    spin_secret_btn: "SPIN (secret mode)",
    no_agent_available: "No agent available",
    language: "Language",
    
    update_available: "Update available",
    download: "Download",
    downloading: "Downloading...",
    download_success: "Downloaded to your Downloads folder!",
    download_error: "Download failed",
    
    role_duelist: "Duelist",
    role_initiator: "Initiator",
    role_controller: "Controller",
    role_sentinel: "Sentinel",
    role_unknown: "Undefined",
};

static ES: Translations = Translations {
    mode_secret: "Modo secreto",
    secret_hint: "Ctrl+clic o Alt+clic en GIRAR",
    target: "Objetivo:",
    add_agent: "Añadir un agente",
    agent_name: "Nombre del agente",
    role: "Rol:",
    custom_role: "Rol personalizado (opcional)",
    choose_image: "Elegir una imagen",
    add_btn: "AÑADIR",
    filters: "Filtros por rol",
    excluded_agents: "Agentes excluidos",
    no_excluded_agent: "Ningún agente excluido",
    restore_all: "↺ RESTAURAR TODO",
    my_agents: "Mis agentes",
    delete_btn: "Eliminar",
    exclude_btn: "EXCLUIR",
    spin_btn: "GIRAR",
    spin_secret_btn: "GIRAR (modo secreto)",
    no_agent_available: "Ningún agente disponible",
    language: "Idioma",
    
    update_available: "Actualización disponible",
    download: "Descargar",
    downloading: "Descargando...",
    download_success: "¡Descargado en tu carpeta de Descargas!",
    download_error: "Error en la descarga",
    
    role_duelist: "Duelista",
    role_initiator: "Iniciador",
    role_controller: "Controlador",
    role_sentinel: "Centinela",
    role_unknown: "Indefinido",
};