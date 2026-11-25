# 🛠️ Commandes essentielles — Valoroulette Rust

[English](COMMANDS.md) | [Français](COMMANDS.fr.md) | [Español](COMMANDS.es.md)

Ce fichier regroupe les commandes utiles au quotidien pour le développement et la maintenance du projet.

---

## 🦀 Cargo (Rust)

| Commande | Description |
| :--- | :--- |
| `cargo run` | Compile et lance l'application en mode debug (rapide, mais binaire lourd). |
| `cargo build` | Compile le projet en mode debug sans l'exécuter. |
| `cargo build --release` | Compile en mode release (optimisé, binaire final pour la distribution). |
| `cargo check` | Vérifie que le code compile **sans** générer de binaire (très rapide). |
| `cargo clippy` | Lance le linter pour détecter les mauvaises pratiques et améliorations possibles. |
| `cargo fmt` | Formate automatiquement tout le code selon les conventions Rust. |
| `cargo update` | Met à jour les dépendances vers leurs dernières versions compatibles. |
| `cargo clean` | Supprime le dossier `target/` (libère de l'espace disque). |

---

## 📦 Git

| Commande | Description |
| :--- | :--- |
| `git status` | Affiche l'état des fichiers (modifiés, indexés, non suivis). |
| `git add .` | Ajoute toutes les modifications à l'index (avant commit). |
| `git commit -m "message"` | Enregistre les modifications indexées avec un message. |
| `git push` | Envoie les commits locaux vers GitHub. |
| `git pull` | Récupère les derniers commits depuis GitHub. |
| `git log --oneline -10` | Affiche les 10 derniers commits de manière condensée. |
| `git tag -a v0.1.0 -m "Release 0.1.0"` | Crée un tag annoté pour une version. |
| `git push origin v0.1.0` | Pousse un tag spécifique vers GitHub. |

---

## 🚀 Releases (GitHub CLI)

| Commande | Description |
| :--- | :--- |
| `gh auth login` | Connecte GitHub CLI à votre compte (à faire une seule fois). |
| `gh release create v0.2.0 --generate-notes` | Crée une release GitHub avec notes auto-générées. |
| `gh release list` | Liste toutes les releases du dépôt. |
| `gh release view` | Affiche les détails de la dernière release. |
| `gh release delete v0.1.0` | Supprime une release (attention, le tag reste). |

---

## 📜 Scripts du projet

| Commande | Description |
| :--- | :--- |
| `bash scripts/release.sh` | Lance le script de release interactif (Git Bash / Linux / macOS). |
| `powershell -File scripts/release.ps1` | Lance le script de release sous PowerShell (Windows). |

---

## 🧹 Nettoyage

| Commande | Description |
| :--- | :--- |
| `rm -rf target/` ou `cargo clean` | Supprime les artefacts de compilation. |
| `git rm -r --cached target` | Retire `target/` de l'index Git (sans le supprimer du disque). |

---

## 💡 Astuces

- Toujours lancer `cargo fmt` et `cargo clippy` avant de commit.
- Le fichier `.gitignore` doit contenir `/target/` pour éviter de versionner les binaires.
- Utiliser `cargo check` plutôt que `cargo build` pendant le développement pour gagner du temps.