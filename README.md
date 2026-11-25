# 🎡 Valoroulette Rust

[English](README.md) | [Français](README.fr.md) | [Español](README.es.md)

A Rust rewrite of [Valoroulette-standalone](https://github.com/Stevehunn/Valoroulette-standalone.git), originally developed in Electron/JavaScript.

Valoroulette is an interactive roulette to randomly select a Valorant agent, with exclusion management, secret mode, and customization.

## ✨ Features

- **Interactive roulette** with agent portraits and smooth animation.
- **Custom agent addition** (name, role, image).
- **Quick exclusion**: click on an agent in the wheel to exclude them.
- **Role filters**: temporarily hide agent categories.
- **Secret mode**: force the result towards a target agent via `Ctrl+click` or `Alt+click` on the SPIN button.
- **Internationalization**: English, French, Spanish.
- **Persistence** of settings and custom agents.
- **Update notifications** directly in the application.

## 🚀 Installation

Download the latest version from the [Releases](https://github.com/Stevehunn/Valoroulette-rust/releases/) page.
Extract the archive and run `valoroulette.exe`.

## 📦 Build from source

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or higher).

### Steps

```bash
git clone https://github.com/Stevehunn/Valoroulette-rust.git
cd Valoroulette-rust
cargo build --release
```

The executable will be located in `target/release/valoroulette.exe`.

## 🔄 Updates

The application automatically checks for new versions on startup via the GitHub Releases API.
If an update is available, a notification appears in the interface with a download button.

## 🧑‍💻 Development

See the [COMMANDS.md](COMMANDS.md) file for the list of useful commands.

## 📝 Changelog

All notable changes are documented in the [CHANGELOG.md](CHANGELOG.md) file.

## 📄 License

This project is distributed under the MIT License. See the `LICENSE` file for more details.