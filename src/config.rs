use serde::{Deserialize, Serialize};

use crate::settings::SettingsLoader;

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
