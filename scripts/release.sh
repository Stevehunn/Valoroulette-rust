#!/usr/bin/env bash
set -e

# --- CONFIGURATION ---
REPO="Stevehunn/Valoroulette-rust"
CARGO_FILE="Cargo.toml"

# --- VÉRIFICATION DES OUTILS ---
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) n'est pas installé. Installez-le : https://cli.github.com/"
    exit 1
fi

if ! gh auth status &> /dev/null; then
    echo "❌ Vous n'êtes pas connecté à GitHub CLI. Lancez : gh auth login"
    exit 1
fi

# --- DEMANDE DE LA VERSION ---
read -p "Nouvelle version (ex: 0.2.0) : " VERSION
if [[ -z "$VERSION" ]]; then
    echo "❌ Version vide. Abandon."
    exit 1
fi
TAG="v$VERSION"

# --- VÉRIFICATION SI LE TAG EXISTE DÉJÀ ---
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "⚠️  Le tag $TAG existe déjà localement."
    read -p "Continuer quand même (la release sera mise à jour) ? [y/N] " CONFIRM
    if [[ ! "$CONFIRM" =~ ^[yY]$ ]]; then
        echo "❌ Abandon."
        exit 1
    fi
fi

# --- MISE À JOUR DE CARGO.TOML ---
echo "📝 Mise à jour de Cargo.toml vers $VERSION..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$CARGO_FILE"

# --- VÉRIFICATION COMPILATION ---
echo "🔨 Compilation de vérification..."
cargo check

# --- GIT COMMIT & TAG ---
echo "📦 Commit et tag Git..."
git add "$CARGO_FILE" CHANGELOG.md 2>/dev/null || true

# Commit seulement s'il y a des changements
if ! git diff --cached --quiet; then
    git commit -m "chore(release): v$VERSION"
else
    echo "ℹ️  Aucun changement à commiter (version déjà à jour)."
fi

# Créer le tag s'il n'existe pas déjà
if ! git rev-parse "$TAG" >/dev/null 2>&1; then
    git tag -a "$TAG" -m "Release $TAG"
fi

git push origin main
git push origin "$TAG"

# --- CRÉATION DE LA RELEASE GITHUB ---
echo "🚀 Création de la release GitHub..."
if gh release view "$TAG" --repo "$REPO" &> /dev/null; then
    echo "ℹ️  Release existante, mise à jour du titre..."
    gh release edit "$TAG" --repo "$REPO" --title "Valoroulette $TAG"
else
    gh release create "$TAG" \
        --repo "$REPO" \
        --title "Valoroulette $TAG" \
        --generate-notes
fi

echo "✅ Release $TAG publiée avec succès !"
echo "🔗 https://github.com/$REPO/releases/tag/$TAG"