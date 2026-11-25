// Embedded agent images, so the exe works anywhere without external assets.
pub fn get_agent_image_bytes(name: &str) -> Option<&'static [u8]> {
    match name {
        "Jett" => Some(include_bytes!("../assets/agents/Jett.png")),
        "Phoenix" => Some(include_bytes!("../assets/agents/Phoenix.png")),
        "Sova" => Some(include_bytes!("../assets/agents/Sova.png")),
        "Sage" => Some(include_bytes!("../assets/agents/Sage.png")),
        "Cypher" => Some(include_bytes!("../assets/agents/Cypher.png")),
        "Omen" => Some(include_bytes!("../assets/agents/Omen.png")),
        "Brimstone" => Some(include_bytes!("../assets/agents/Brimstone.png")),
        "Viper" => Some(include_bytes!("../assets/agents/Viper.png")),
        "Killjoy" => Some(include_bytes!("../assets/agents/Killjoy.png")),
        "Raze" => Some(include_bytes!("../assets/agents/Raze.png")),
        "Breach" => Some(include_bytes!("../assets/agents/Breach.png")),
        "Skye" => Some(include_bytes!("../assets/agents/Skye.png")),
        "Yoru" => Some(include_bytes!("../assets/agents/Yoru.png")),
        "KAY/O" => Some(include_bytes!("../assets/agents/Kayo.png")),
        "Astra" => Some(include_bytes!("../assets/agents/Astra.png")),
        "Neon" => Some(include_bytes!("../assets/agents/Neon.png")),
        "Harbor" => Some(include_bytes!("../assets/agents/Harbor.png")),
        "Chamber" => Some(include_bytes!("../assets/agents/Chamber.png")),
        "Fade" => Some(include_bytes!("../assets/agents/Fade.png")),
        "Gekko" => Some(include_bytes!("../assets/agents/Gekko.png")),
        "Deadlock" => Some(include_bytes!("../assets/agents/Deadlock.png")),
        "Clove" => Some(include_bytes!("../assets/agents/Clove.png")),
        "Vyse" => Some(include_bytes!("../assets/agents/Vyse.png")),
        "Tejo" => Some(include_bytes!("../assets/agents/Tejo.png")),
        "Reyna" => Some(include_bytes!("../assets/agents/Reyna.png")),
        "Iso" => Some(include_bytes!("../assets/agents/Iso.png")),
        "Waylay" => Some(include_bytes!("../assets/agents/Waylay.png")),
        "Veto" => Some(include_bytes!("../assets/agents/Veto.png")),
        "Miks" => Some(include_bytes!("../assets/agents/Miks.png")),
        "Hush" => Some(include_bytes!("../assets/agents/Hush.png")),
        _ => None,
    }
}

pub fn get_icon_bytes() -> &'static [u8] {
    include_bytes!("../assets/icon.ico")
}