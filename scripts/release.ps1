$ErrorActionPreference = "Stop"

$Repo = "Stevehunn/Valoroulette-rust"
$CargoFile = "Cargo.toml"

if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "❌ GitHub CLI (gh) n'est pas installé." -ForegroundColor Red
    exit 1
}

$Version = Read-Host "Nouvelle version (ex: 0.2.0)"
if ([string]::IsNullOrWhiteSpace($Version)) {
    Write-Host "❌ Version vide. Abandon." -ForegroundColor Red
    exit 1
}
$Tag = "v$Version"

Write-Host "📝 Mise à jour de Cargo.toml vers $Version..."
(Get-Content $CargoFile) -replace '^version = ".*"', "version = `"$Version`"" | Set-Content $CargoFile

Write-Host "🔨 Compilation de vérification..."
cargo check

Write-Host "📦 Commit et tag Git..."
git add $CargoFile CHANGELOG.md
git commit -m "chore(release): v$Version"
git tag -a $Tag -m "Release $Tag"
git push origin main
git push origin $Tag

Write-Host "🚀 Création de la release GitHub..."
gh release create $Tag `
    --repo $Repo `
    --title "Valoroulette $Tag" `
    --notes-from-tag `
    --generate-notes

Write-Host "✅ Release $Tag publiée avec succès !" -ForegroundColor Green
Write-Host "🔗 https://github.com/$Repo/releases/tag/$Tag"