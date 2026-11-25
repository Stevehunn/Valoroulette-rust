# Changelog

[English](CHANGELOG.md) | [Français](CHANGELOG.fr.md) | [Español](CHANGELOG.es.md)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.2.2] - 2024-05-26

### Added
- Direct `.exe` download from the update banner, saving the file to the user's Downloads folder.
- Visual feedback in the update banner (downloading status, success and error messages).

### Changed
- Removed the top logo banner for a cleaner and more modern interface.
- Redesigned the update banner button to a clean white style for better visibility.

### Fixed
- Hidden the console window when launching the release executable on Windows.
- Embedded agent images directly into the binary so they display correctly anywhere.
- Added a proper Windows `.ico` icon for the `.exe` file in the file explorer.

## [0.2.1] - 2024-05-25

### Fixed
- Hidden the console window when launching the release executable on Windows.
- Embedded agent images directly into the binary so they display correctly anywhere.
- Added a proper Windows `.ico` icon for the `.exe` file.
- Removed the top logo banner for a cleaner interface.

### Added
- Release scripts (`release.sh` and `release.ps1`) for automated publishing.

## [0.1.0] - 2024-05-24

### Added
- Interactive roulette with agent portraits and spin animation.
- Stylized dark interface (Valorant palette) with a resizable side panel.
- Agent management: custom addition (name, role, image), deletion, and exclusion.
- Quick agent exclusion by clicking on their portrait in the wheel.
- Individual and global restoration of excluded agents.
- Secret mode activated via `Ctrl+click` or `Alt+click` on the SPIN button.
- Display of the winner's portrait and name in the center of the wheel.
- Full internationalization (English, French, Spanish).
- Persistence of settings and agents in the OS configuration folder.
- Custom window icon and in-app logo.
- Update notification (GitHub Releases API polling).