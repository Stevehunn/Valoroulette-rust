# Changelog

[English](CHANGELOG.md) | [Français](CHANGELOG.fr.md) | [Español](CHANGELOG.es.md)

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [0.2.1] - 2024-05-25

### Corrigé
- Masquage de la fenêtre de console lors du lancement de l'exécutable en mode release sous Windows.
- Intégration des images des agents directement dans le binaire pour qu'elles s'affichent correctement peu importe l'emplacement de l'exécutable.
- Ajout d'une véritable icône `.ico` Windows pour le fichier `.exe` dans l'explorateur.
- Suppression du bandeau supérieur avec le logo pour une interface plus épurée.

### Ajouté
- Scripts de release (`release.sh` et `release.ps1`) pour automatiser la publication sur GitHub.

## [0.1.0] - 2024-05-24

### Ajouté
- Roulette interactive avec portraits d'agents et animation de rotation.
- Interface sombre stylisée (palette Valorant) avec panneau latéral redimensionnable.
- Gestion des agents : ajout personnalisé (nom, rôle, image), suppression et exclusion.
- Exclusion rapide d'un agent en cliquant sur son portrait dans la roue.
- Restauration individuelle et globale des agents exclus.
- Mode secret activable via `Ctrl+clic` ou `Alt+clic` sur le bouton LANCER.
- Affichage du portrait et du nom du gagnant au centre de la roue.
- Internationalisation complète (Français, Anglais, Espagnol).
- Persistance des réglages et des agents dans le dossier de configuration de l'OS.
- Icône de fenêtre personnalisée et logo dans l'application.
- Notification de mise à jour disponible (interrogation de l'API GitHub Releases).