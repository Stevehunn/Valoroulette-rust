# 🛠️ Essential Commands — Valoroulette Rust

[English](COMMANDS.md) | [Français](COMMANDS.fr.md) | [Español](COMMANDS.es.md)

This file gathers useful commands for the daily development and maintenance of the project.

---

## 🦀 Cargo (Rust)

| Command | Description |
| :--- | :--- |
| `cargo run` | Compiles and runs the app in debug mode (fast, but heavy binary). |
| `cargo build` | Compiles the project in debug mode without running it. |
| `cargo build --release` | Compiles in release mode (optimized, final binary for distribution). |
| `cargo check` | Checks that the code compiles **without** generating a binary (very fast). |
| `cargo clippy` | Runs the linter to detect bad practices and possible improvements. |
| `cargo fmt` | Automatically formats all code according to Rust conventions. |
| `cargo update` | Updates dependencies to their latest compatible versions. |
| `cargo clean` | Deletes the `target/` folder (frees up disk space). |

---

## 📦 Git

| Command | Description |
| :--- | :--- |
| `git status` | Shows the state of files (modified, staged, untracked). |
| `git add .` | Stages all modifications (before commit). |
| `git commit -m "message"` | Records staged changes with a message. |
| `git push` | Sends local commits to GitHub. |
| `git pull` | Fetches the latest commits from GitHub. |
| `git log --oneline -10` | Shows the last 10 commits in a condensed format. |
| `git tag -a v0.1.0 -m "Release 0.1.0"` | Creates an annotated tag for a version. |
| `git push origin v0.1.0` | Pushes a specific tag to GitHub. |

---

## 🚀 Releases (GitHub CLI)

| Command | Description |
| :--- | :--- |
| `gh auth login` | Connects GitHub CLI to your account (only needed once). |
| `gh release create v0.2.0 --generate-notes` | Creates a GitHub release with auto-generated notes. |
| `gh release list` | Lists all releases in the repository. |
| `gh release view` | Displays details of the latest release. |
| `gh release delete v0.1.0` | Deletes a release (warning: the tag remains). |

---

## 📜 Project Scripts

| Command | Description |
| :--- | :--- |
| `bash scripts/release.sh` | Runs the interactive release script (Git Bash / Linux / macOS). |
| `powershell -File scripts/release.ps1` | Runs the release script under PowerShell (Windows). |

---

## 🧹 Cleanup

| Command | Description |
| :--- | :--- |
| `rm -rf target/` or `cargo clean` | Deletes compilation artifacts. |
| `git rm -r --cached target` | Removes `target/` from the Git index (without deleting it from disk). |

---

## 💡 Tips

- Always run `cargo fmt` and `cargo clippy` before committing.
- The `.gitignore` file must contain `/target/` to avoid versioning binaries.
- Use `cargo check` instead of `cargo build` during development to save time.