use serde::Deserialize;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;

const REPO: &str = "Stevehunn/Valoroulette-rust";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct ReleaseInfo {
    tag_name: String,
    html_url: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub latest_version: String,
    pub release_url: String,
    pub download_url: Option<String>,
}

pub struct UpdateChecker {
    check_receiver: Option<Receiver<Option<UpdateInfo>>>,
    download_receiver: Option<Receiver<Result<String, String>>>,
    pub result: Option<UpdateInfo>,
    pub checked: bool,
    pub is_downloading: bool,
    pub download_status: Option<String>,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            check_receiver: None,
            download_receiver: None,
            result: None,
            checked: false,
            is_downloading: false,
            download_status: None,
        }
    }

    /// Start checking for updates in the background.
    pub fn check_async(&mut self) {
        if self.check_receiver.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.check_receiver = Some(rx);
        self.checked = false;

        thread::spawn(move || {
            let info = fetch_latest_release();
            let _ = tx.send(info);
        });
    }

    /// Start downloading the executable in the background.
    pub fn start_download(&mut self, url: String) {
        if self.download_receiver.is_some() {
            return;
        }
        let (tx, rx) = mpsc::channel();
        self.download_receiver = Some(rx);
        self.is_downloading = true;
        self.download_status = None;

        thread::spawn(move || {
            let result = download_file(&url);
            let _ = tx.send(result);
        });
    }

    /// Poll both the check and download channels.
    pub fn poll(&mut self) {
        // Poll check
        if let Some(rx) = &self.check_receiver {
            match rx.try_recv() {
                Ok(info) => {
                    self.result = info;
                    self.checked = true;
                    self.check_receiver = None;
                }
                Err(TryRecvError::Disconnected) => {
                    self.checked = true;
                    self.check_receiver = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }

        // Poll download
        if let Some(rx) = &self.download_receiver {
            match rx.try_recv() {
                Ok(Ok(path)) => {
                    self.is_downloading = false;
                    self.download_status = Some(format!("OK:{}", path));
                    self.download_receiver = None;
                }
                Ok(Err(e)) => {
                    self.is_downloading = false;
                    self.download_status = Some(format!("ERR:{}", e));
                    self.download_receiver = None;
                }
                Err(TryRecvError::Disconnected) => {
                    self.is_downloading = false;
                    self.download_receiver = None;
                }
                Err(TryRecvError::Empty) => {}
            }
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
        // Find the .exe asset
        let download_url = release
            .assets
            .iter()
            .find(|a| a.name.ends_with(".exe"))
            .map(|a| a.browser_download_url.clone());

        Some(UpdateInfo {
            latest_version: remote_version,
            release_url: release.html_url,
            download_url,
        })
    } else {
        None
    }
}

fn download_file(url: &str) -> Result<String, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("Valoroulette-Rust-Updater")
        .build()
        .map_err(|e| e.to_string())?;

    let mut resp = client.get(url).send().map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }

    // Save to Downloads folder
    let download_dir = dirs::download_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let file_path = download_dir.join("valoroulette_update.exe");

    let mut file = std::fs::File::create(&file_path).map_err(|e| e.to_string())?;
    std::io::copy(&mut resp, &mut file).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().to_string())
}

/// Simple semantic version comparison (x.y.z).
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