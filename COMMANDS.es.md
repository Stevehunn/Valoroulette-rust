# 🛠️ Comandos esenciales — Valoroulette Rust

[English](COMMANDS.md) | [Français](COMMANDS.fr.md) | [Español](COMMANDS.es.md)

Este archivo reúne los comandos útiles para el desarrollo y mantenimiento diario del proyecto.

---

## 🦀 Cargo (Rust)

| Comando | Descripción |
| :--- | :--- |
| `cargo run` | Compila y ejecuta la aplicación en modo debug (rápido, pero binario pesado). |
| `cargo build` | Compila el proyecto en modo debug sin ejecutarlo. |
| `cargo build --release` | Compila en modo release (optimizado, binario final para distribución). |
| `cargo check` | Verifica que el código compile **sin** generar un binario (muy rápido). |
| `cargo clippy` | Ejecuta el linter para detectar malas prácticas y posibles mejoras. |
| `cargo fmt` | Formatea automáticamente todo el código según las convenciones de Rust. |
| `cargo update` | Actualiza las dependencias a sus últimas versiones compatibles. |
| `cargo clean` | Elimina la carpeta `target/` (libera espacio en disco). |

---

## 📦 Git

| Comando | Descripción |
| :--- | :--- |
| `git status` | Muestra el estado de los archivos (modificados, en stage, no rastreados). |
| `git add .` | Añade todas las modificaciones al stage (antes del commit). |
| `git commit -m "mensaje"` | Registra los cambios en stage con un mensaje. |
| `git push` | Envía los commits locales a GitHub. |
| `git pull` | Recupera los últimos commits desde GitHub. |
| `git log --oneline -10` | Muestra los últimos 10 commits de forma condensada. |
| `git tag -a v0.1.0 -m "Release 0.1.0"` | Crea una etiqueta anotada para una versión. |
| `git push origin v0.1.0` | Sube una etiqueta específica a GitHub. |

---

## 🚀 Releases (GitHub CLI)

| Comando | Descripción |
| :--- | :--- |
| `gh auth login` | Conecta GitHub CLI a tu cuenta (solo es necesario una vez). |
| `gh release create v0.2.0 --generate-notes` | Crea una release en GitHub con notas autogeneradas. |
| `gh release list` | Lista todas las releases del repositorio. |
| `gh release view` | Muestra los detalles de la última release. |
| `gh release delete v0.1.0` | Elimina una release (atención: la etiqueta permanece). |

---

## 📜 Scripts del proyecto

| Comando | Descripción |
| :--- | :--- |
| `bash scripts/release.sh` | Ejecuta el script de release interactivo (Git Bash / Linux / macOS). |
| `powershell -File scripts/release.ps1` | Ejecuta el script de release en PowerShell (Windows). |

---

## 🧹 Limpieza

| Comando | Descripción |
| :--- | :--- |
| `rm -rf target/` o `cargo clean` | Elimina los artefactos de compilación. |
| `git rm -r --cached target` | Elimina `target/` del índice de Git (sin borrarlo del disco). |

---

## 💡 Consejos

- Ejecuta siempre `cargo fmt` y `cargo clippy` antes de hacer commit.
- El archivo `.gitignore` debe contener `/target/` para evitar versionar binarios.
- Usa `cargo check` en lugar de `cargo build` durante el desarrollo para ahorrar tiempo.