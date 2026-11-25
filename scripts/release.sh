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

# --- MISE À JOUR DE CARGO.TOML ---
echo "📝 Mise à jour de Cargo.toml vers $VERSION..."
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" "$CARGO_FILE"

# --- VÉRIFICATION COMPILATION ---
echo "🔨 Compilation de vérification..."
cargo check

# --- GIT COMMIT & TAG ---
echo "📦 Commit et tag Git..."
git add "$CARGO_FILE" CHANGELOG.md
git commit -m "chore(release): v$VERSION"
git tag -a "$TAG" -m "Release $TAG"
git push origin main
git push origin "$TAG"

# --- CRÉATION DE LA RELEASE GITHUB ---
echo "🚀 Création de la release GitHub..."
gh release create "$TAG" \
    --repo "$REPO" \
    --title "Valoroulette $TAG" \
    --notes-from-tag \
    --generate-notes

echo "✅ Release $TAG publiée avec succès !"
echo "🔗 https://github.com/$REPO/releases/tag/$TAG"