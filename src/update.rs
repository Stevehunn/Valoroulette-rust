use serde::Deserialize;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

const REPO: &str = "Stevehunn/Valoroulette-rust";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
struct ReleaseInfo {
    tag_name: String,
    html_url: String,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub latest_version: String,
    pub release_url: String,
}

pub struct UpdateChecker {
    receiver: Option<Receiver<Option<UpdateInfo>>>,
    pub result: Option<UpdateInfo>,
    pub checked: bool,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            receiver: None,
            result: None,
            checked: false,
        }
    }

    /// Lance la vérification en arrière-plan (non bloquant pour l'UI).
    pub fn check_async(&mut self) {
        if self.receiver.is_some() {
            return; // Déjà en cours
        }
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        self.checked = false;

        thread::spawn(move || {
            let info = fetch_latest_release();
            let _ = tx.send(info);
        });
    }

    /// À appeler à chaque frame pour récupérer le résultat quand il est prêt.
    pub fn poll(&mut self) {
        let Some(rx) = &self.receiver else { return };
        match rx.try_recv() {
            Ok(info) => {
                self.result = info;
                self.checked = true;
                self.receiver = None;
            }
            Err(TryRecvError::Disconnected) => {
                self.checked = true;
                self.receiver = None;
            }
            Err(TryRecvError::Empty) => {}
        }
    }

    pub fn update_available(&self) -> bool {
        self.result.is_some()
    }
}

fn fetch_latest_release() -> Option<UpdateInfo> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let client = reqwest::blocking::Client::builder()
        .user_agent("Valoroulette-Rust-Updater")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?;

    let resp = client.get(&url).send().ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let release: ReleaseInfo = resp.json().ok()?;

    let remote_version = release.tag_name.trim_start_matches('v').to_string();
    let local_version = CURRENT_VERSION.to_string();

    if is_newer(&remote_version, &local_version) {
        Some(UpdateInfo {
            latest_version: remote_version,
            release_url: release.html_url,
        })
    } else {
        None
    }
}

/// Comparaison sémantique simple (x.y.z).
fn is_newer(remote: &str, local: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|p| p.parse().ok())
            .collect()
    };
    let r = parse(remote);
    let l = parse(local);
    for i in 0..r.len().max(l.len()) {
        let rv = r.get(i).copied().unwrap_or(0);
        let lv = l.get(i).copied().unwrap_or(0);
        if rv > lv { return true; }
        if rv < lv { return false; }
    }
    false
}