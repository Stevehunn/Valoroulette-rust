# Valoroulette Rust

Réécriture native de Valoroulette avec Rust + `egui/eframe`.

## Prérequis

- Rust stable et Cargo : https://www.rust-lang.org/tools/install
- Sous Windows : Visual Studio Build Tools avec le composant **Desktop development with C++** (toolchain MSVC).

## Lancer en développement

```bash
cargo run
```

## Compiler

```bash
cargo build --release
```

L'exécutable est généré dans `target/release/valoroulette.exe` sous Windows.

## État du portage

Cette première base contient :
- fenêtre native `eframe`;
- roue dessinée directement avec le painter egui;
- animation de rotation;
- catalogue des 30 entrées du fichier `agents.json` du projet d'origine;
- filtres par rôle et exclusions individuelles;
- ajout d'un agent personnalisé (nom uniquement à ce stade);
- sauvegarde locale des exclusions et paramètres.

À compléter dans les prochaines étapes :
- intégrer les portraits PNG et le logo d'origine;
- aligner exactement le comportement du Secret Mode sur l'application Electron;
- persister les agents personnalisés et leurs portraits;
- importer les 10 traductions existantes;
- mettre en place la vérification et l'installation des mises à jour GitHub Releases;
- tests fonctionnels et packaging Windows.

## Notes de migration

Le rôle de `Veto` n'était pas défini dans la table des rôles du renderer JavaScript d'origine. Il est donc classé `Unknown` ici, plutôt que de lui attribuer arbitrairement un rôle.
