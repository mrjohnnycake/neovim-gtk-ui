use log::error;
use serde::{Deserialize, Serialize};

use crate::dirs;
use crate::settings::SettingsLoader;

/// Written to config.toml the first time the app runs and no config.toml
/// exists yet. Every line is commented out and shows its default, so the
/// file is both documentation and a ready-to-edit starting point.
const CONFIG_TEMPLATE: &str = r#"# --- Neovim GTK UI configuration --- #

# Uncomment a line and change its value to override the default shown.
# A matching NVIM_GTK_* environment variable, if set, wins over this file.

# Use the dark variant of the GTK theme.
# prefer_dark_theme = true

# Show the header bar (title, menu, buttons) at the top of the window.
# show_header_bar = true

# Show the window's title bar and border.
# window_decorations = true

# Show the file browser sidebar. Left commented, this remembers whatever
# you last left it as; set explicitly to always start the same way.
# show_sidebar = true

# Editor font, as a Pango font description, e.g. "Iosevka 14".
# Left commented, this follows the GNOME system monospace font.
# font = "Monospace 12"

# OpenType font features, e.g. "cv17, ss01" for stylistic sets.
# font_features = "cv17"

# Extra pixels added between lines. Can be negative.
# linespace = 0

# Window transparency, from 0.0 (invisible) to 1.0 (opaque).
# transparency = 1.0

# How many times the cursor blinks before it stops; -1 blinks forever.
# cursor_blink = -1

# Render the completion popup as a native GTK widget instead of nvim's
# own terminal-style popup.
# external_popupmenu = true

# Render the tab bar as a native GTK widget instead of nvim's own tabline.
# external_tabline = true

# Render the command line as a native GTK widget instead of nvim's own
# terminal-style command line.
# external_cmdline = false

# Use the GTK clipboard directly for the + and * registers, instead of
# nvim's own clipboard provider (e.g. xclip/wl-clipboard).
# internal_clipboard = false
"#;

/// User-facing app settings, loaded from `config.toml` in the app config
/// directory. Each field can also be overridden per-invocation by the
/// matching `NVIM_GTK_*` environment variable.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub prefer_dark_theme: bool,
    pub show_header_bar: bool,
    pub window_decorations: bool,
    pub show_sidebar: Option<bool>,
    pub font: Option<String>,
    pub font_features: Option<String>,
    pub linespace: Option<i32>,
    pub transparency: Option<f64>,
    pub cursor_blink: Option<i32>,
    pub external_popupmenu: Option<bool>,
    pub external_tabline: Option<bool>,
    pub external_cmdline: Option<bool>,
    pub internal_clipboard: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            prefer_dark_theme: true,
            show_header_bar: true,
            window_decorations: true,
            show_sidebar: None,
            font: None,
            font_features: None,
            linespace: None,
            transparency: None,
            cursor_blink: None,
            external_popupmenu: None,
            external_tabline: None,
            external_cmdline: None,
            internal_clipboard: false,
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

    /// Ex commands (using the same GuiXxx/NGXxx commands ginit.vim would use)
    /// that apply every setting here which isn't left at its default.
    pub fn ex_commands(&self) -> Vec<String> {
        let mut commands = Vec::new();

        if let Some(features) = &self.font_features {
            commands.push(format!("GuiFontFeatures {features}"));
        }
        if let Some(linespace) = self.linespace {
            commands.push(format!("GuiLinespace {linespace}"));
        }
        if let Some(alpha) = self.transparency {
            commands.push(format!("NGTransparency {alpha} {alpha}"));
        }
        if let Some(blink) = self.cursor_blink {
            commands.push(format!("NGSetCursorBlink {blink}"));
        }
        if let Some(enabled) = self.external_popupmenu {
            commands.push(format!("GuiPopupmenu {}", enabled as u8));
        }
        if let Some(enabled) = self.external_tabline {
            commands.push(format!("GuiTabline {}", enabled as u8));
        }
        if let Some(enabled) = self.external_cmdline {
            commands.push(format!("GuiCmdline {}", enabled as u8));
        }

        commands
    }
}
