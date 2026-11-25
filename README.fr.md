# 🎡 Valoroulette Rust

[English](README.md) | [Français](README.fr.md) | [Español](README.es.md)

Une réécriture en Rust de [Valoroulette-standalone](https://github.com/Stevehunn/Valoroulette-standalone.git), initialement développé en Electron/JavaScript.

Valoroulette est une roulette interactive pour sélectionner aléatoirement un agent de Valorant, avec gestion des exclusions, mode secret et personnalisation.

## ✨ Fonctionnalités

- **Roulette interactive** avec portraits d'agents et animation fluide.
- **Ajout personnalisé** d'agents (nom, rôle, image).
- **Exclusion rapide** : cliquez sur un agent dans la roue pour l'exclure.
- **Filtres par rôle** : masquez temporairement des catégories d'agents.
- **Mode secret** : forcez le résultat vers un agent cible via `Ctrl+clic` ou `Alt+clic` sur le bouton LANCER.
- **Internationalisation** : Français, Anglais, Espagnol.
- **Persistance** des réglages et des agents personnalisés.
- **Notifications de mise à jour** directement dans l'application.

## 🚀 Installation

Téléchargez la dernière version depuis la page [Releases](https://github.com/Stevehunn/Valoroulette-rust/releases/).
Décompressez l'archive et lancez `valoroulette.exe`.

## 📦 Compilation depuis les sources

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou supérieure).

### Étapes

```bash
git clone https://github.com/Stevehunn/Valoroulette-rust.git
cd Valoroulette-rust
cargo build --release
```

L'exécutable se trouvera dans `target/release/valoroulette.exe`.

## 🔄 Mises à jour

L'application vérifie automatiquement les nouvelles versions au démarrage via l'API GitHub Releases.
Si une mise à jour est disponible, une notification apparaît dans l'interface avec un bouton de téléchargement.

## 🧑‍💻 Développement

Consultez le fichier [COMMANDS.md](COMMANDS.md) pour la liste des commandes utiles au quotidien.

## 📝 Changelog

Toutes les modifications notables sont documentées dans le fichier [CHANGELOG.md](CHANGELOG.md).

## 📄 Licence

Ce projet est distribué sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.