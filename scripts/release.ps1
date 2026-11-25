$ErrorActionPreference = "Stop"

# --- CONFIGURATION ---
$Repo = "Stevehunn/Valoroulette-rust"
$CargoFile = "Cargo.toml"

# --- VÉRIFICATION DES OUTILS ---
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ GitHub CLI (gh) n'est pas installé." -ForegroundColor Red
    exit 1
}

# --- DEMANDE DE LA VERSION ---
$Version = Read-Host "Nouvelle version (ex: 0.2.0)"
if ([string]::IsNullOrWhiteSpace($Version)) {
    Write-Host "❌ Version vide. Abandon." -ForegroundColor Red
    exit 1
}
$Tag = "v$Version"

# --- VÉRIFICATION SI LE TAG EXISTE DÉJÀ ---
$existingTag = git tag -l $Tag
if ($existingTag) {
    Write-Host "⚠️  Le tag $Tag existe déjà localement." -ForegroundColor Yellow
    $confirm = Read-Host "Continuer quand même (la release sera mise à jour) ? [y/N]"
    if ($confirm -notmatch "^[yY]$") {
        Write-Host "❌ Abandon." -ForegroundColor Red
        exit 1
    }
}

# --- MISE À JOUR DE CARGO.TOML ---
Write-Host "📝 Mise à jour de Cargo.toml vers $Version..."
(Get-Content $CargoFile) -replace '^version = ".*"', "version = `"$Version`"" | Set-Content $CargoFile

# --- VÉRIFICATION COMPILATION ---
Write-Host "🔨 Compilation de vérification..."
cargo check
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ La compilation a échoué. Abandon." -ForegroundColor Red
    exit 1
}

# --- GIT COMMIT & TAG ---
Write-Host "📦 Commit et tag Git..."
git add $CargoFile CHANGELOG.md 2>$null

# Commit seulement s'il y a des changements
$staged = git diff --cached --name-only
if ($staged) {
    git commit -m "chore(release): v$Version"
} else {
    Write-Host "ℹ️  Aucun changement à commiter (version déjà à jour)."
}

# Créer le tag s'il n'existe pas déjà
$tagExists = git tag -l $Tag
if (-not $tagExists) {
    git tag -a $Tag -m "Release $Tag"
}

git push origin main
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Échec du push sur main." -ForegroundColor Red
    exit 1
}

git push origin $Tag
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Échec du push du tag." -ForegroundColor Red
    exit 1
}

# --- CRÉATION DE LA RELEASE GITHUB ---
Write-Host "🚀 Création de la release GitHub..."
gh release view $Tag --repo $Repo 2>$null
if ($LASTEXITCODE -eq 0) {
    Write-Host "ℹ️  Release existante, mise à jour du titre..." -ForegroundColor Yellow
    gh release edit $Tag --repo $Repo --title "Valoroulette $Tag"
} else {
    gh release create $Tag `
        --repo $Repo `
        --title "Valoroulette $Tag" `
        --generate-notes
}

Write-Host "✅ Release $Tag publiée avec succès !" -ForegroundColor Green
Write-Host "🔗 https://github.com/$Repo/releases/tag/$Tag"