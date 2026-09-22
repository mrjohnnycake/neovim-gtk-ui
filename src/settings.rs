use std::cell::RefCell;
use std::rc::{Rc, Weak};

use log::error;

use crate::shell::Shell;
use gio::{self, prelude::*};

#[derive(PartialEq, Eq)]
pub enum FontSource {
    Rpc,
    Config,
    Gnome,
    Default,
}

struct State {
    font_source: FontSource,

    gnome_interface_settings: gio::Settings,
}

impl State {
    pub fn new() -> State {
        State {
            font_source: FontSource::Default,
            gnome_interface_settings: gio::Settings::new("org.gnome.desktop.interface"),
        }
    }

    fn update_font(&mut self, shell: &mut Shell) {
        // rpc and config.toml both take priority over the GNOME default
        if matches!(self.font_source, FontSource::Rpc | FontSource::Config) {
            return;
        }

        shell.set_font_desc(&self.gnome_interface_settings.string("monospace-font-name"));
        self.font_source = FontSource::Gnome;
    }
}

pub struct Settings {
    shell: Option<Weak<RefCell<Shell>>>,
    state: Rc<RefCell<State>>,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            shell: None,
            state: Rc::new(RefCell::new(State::new())),
        }
    }

    pub fn set_shell(&mut self, shell: Weak<RefCell<Shell>>) {
        self.shell = Some(shell);
    }

    pub fn init(&mut self, config_font: Option<&str>) {
        let shell = Weak::upgrade(self.shell.as_ref().unwrap()).unwrap();
        let state = self.state.clone();

        if let Some(font) = config_font {
            shell.borrow_mut().set_font_desc(font);
            self.state.borrow_mut().font_source = FontSource::Config;
        } else {
            self.state.borrow_mut().update_font(&mut shell.borrow_mut());
        }

        self.state
            .borrow()
            .gnome_interface_settings
            .connect_changed(None, move |_, _| {
                monospace_font_changed(&mut shell.borrow_mut(), &mut state.borrow_mut())
            });
    }

    pub fn set_font_source(&mut self, src: FontSource) {
        self.state.borrow_mut().font_source = src;
    }
}

fn monospace_font_changed(shell: &mut Shell, state: &mut State) {
    // rpc and config.toml both take priority over the GNOME default
    if !matches!(state.font_source, FontSource::Rpc | FontSource::Config) {
        state.update_font(shell);
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::dirs;

pub trait SettingsLoader: Sized + serde::Serialize + Default {
    const SETTINGS_FILE: &'static str;

    fn from_str(s: &str) -> Result<Self, String>;

    fn load() -> Self {
        match load_err() {
            Ok(settings) => settings,
            Err(e) => {
                error!("{e}");
                Default::default()
            }
        }
    }

    fn is_file_exists() -> bool {
        dirs::app_config_dir()
            .to_path_buf()
            .join(Self::SETTINGS_FILE)
            .is_file()
    }

    fn save(&self) {
        match save_err(self) {
            Ok(()) => (),
            Err(e) => error!("{e}"),
        }
    }
}

fn load_from_file<T: SettingsLoader>(path: &Path) -> Result<T, String> {
    if path.exists() {
        let mut file = File::open(path).map_err(|e| format!("{e}"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("{e}"))?;
        T::from_str(&contents)
    } else {
        Ok(Default::default())
    }
}

fn load_err<T: SettingsLoader>() -> Result<T, String> {
    let mut toml_path = dirs::app_config_dir_create()?;
    toml_path.push(T::SETTINGS_FILE);
    load_from_file(&toml_path)
}

fn save_err<T: SettingsLoader>(sl: &T) -> Result<(), String> {
    let mut toml_path = dirs::app_config_dir_create()?;
    toml_path.push(T::SETTINGS_FILE);
    let mut file = File::create(toml_path).map_err(|e| format!("{e}"))?;

    let contents = toml::to_string::<T>(sl).map_err(|e| format!("{e}"))?;

    file.write_all(contents.as_bytes())
        .map_err(|e| format!("{e}"))?;

    Ok(())
}
