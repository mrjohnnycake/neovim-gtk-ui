use log::error;
use serde::{Deserialize, Serialize};

use crate::dirs;
use crate::settings::SettingsLoader;

/// Written to config.toml the first time the app runs and no config.toml
/// exists yet. Every line is commented out and shows its default, so the
/// file is both documentation and a ready-to-edit starting point.
const CONFIG_TEMPLATE: &str = r#"# neovim-gtk-ui configuration.
# Uncomment a line and change its value to override the default shown.
# A matching NVIM_GTK_* environment variable, if set, wins over this file.

# Use the dark variant of the GTK theme.
# prefer_dark_theme = false

# Show the header bar (title, menu, buttons) at the top of the window.
# show_header_bar = true

# Show the window's title bar and border.
# window_decorations = true

# Editor font, as a Pango font description, e.g. "Iosevka 14".
# Left commented, this follows the GNOME system monospace font.
# font = "Monospace 12"
"#;

/// User-facing app settings, loaded from `config.toml` in the app config
/// directory. Each field can also be overridden per-invocation by the
/// matching `NVIM_GTK_*` environment variable.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub prefer_dark_theme: bool,
    pub show_header_bar: bool,
    pub window_decorations: bool,
    pub font: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            prefer_dark_theme: false,
            show_header_bar: true,
            window_decorations: true,
            font: None,
        }
    }
}

impl SettingsLoader for AppConfig {
    const SETTINGS_FILE: &'static str = "config.toml";

    fn from_str(s: &str) -> Result<Self, String> {
        toml::from_str(s).map_err(|e| format!("{e}"))
    }
}

impl AppConfig {
    /// Loads config.toml, writing out a commented default template first if
    /// no config.toml exists yet.
    pub fn load_or_init() -> Self {
        if !Self::is_file_exists()
            && let Ok(mut path) = dirs::app_config_dir_create()
        {
            path.push(Self::SETTINGS_FILE);
            if let Err(e) = std::fs::write(&path, CONFIG_TEMPLATE) {
                error!("Failed to write default {}: {e}", Self::SETTINGS_FILE);
            }
        }

        Self::load()
    }
}
