# 🎡 Valoroulette Rust

[English](README.md) | [Français](README.fr.md) | [Español](README.es.md)

Una reescritura en Rust de [Valoroulette-standalone](https://github.com/Stevehunn/Valoroulette-standalone.git), desarrollado originalmente en Electron/JavaScript.

Valoroulette es una ruleta interactiva para seleccionar aleatoriamente un agente de Valorant, con gestión de exclusiones, modo secreto y personalización.

## ✨ Características

- **Ruleta interactiva** con retratos de agentes y animación fluida.
- **Añadir agente personalizado** (nombre, rol, imagen).
- **Exclusión rápida**: haz clic en un agente de la ruleta para excluirlo.
- **Filtros por rol**: oculta temporalmente categorías de agentes.
- **Modo secreto**: fuerza el resultado hacia un agente objetivo mediante `Ctrl+clic` o `Alt+clic` en el botón GIRAR.
- **Internacionalización**: Español, Inglés, Francés.
- **Persistencia** de ajustes y agentes personalizados.
- **Notificaciones de actualización** directamente en la aplicación.

## 🚀 Instalación

Descarga la última versión desde la página de [Releases](https://github.com/Stevehunn/Valoroulette-rust/releases/).
Descomprime el archivo y ejecuta `valoroulette.exe`.

## 📦 Compilación desde el código fuente

### Requisitos previos

- [Rust](https://www.rust-lang.org/tools/install) (edición 2021 o superior).

### Pasos

```bash
git clone https://github.com/Stevehunn/Valoroulette-rust.git
cd Valoroulette-rust
cargo build --release
```

El ejecutable se encontrará en `target/release/valoroulette.exe`.

## 🔄 Actualizaciones

La aplicación verifica automáticamente las nuevas versiones al iniciar mediante la API de GitHub Releases.
Si hay una actualización disponible, aparece una notificación en la interfaz con un botón de descarga.

## 🧑‍💻 Desarrollo

Consulta el archivo [COMMANDS.md](COMMANDS.md) para ver la lista de comandos útiles.

## 📝 Changelog

Todos los cambios notables se documentan en el archivo [CHANGELOG.md](CHANGELOG.md).

## 📄 Licencia

Este proyecto se distribuye bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.