use std::path::PathBuf;
use std::{env, fs, io};

use qt_bindings::RSettings;

pub mod branding {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const APPNAME: &str = env!("CARGO_PKG_NAME");
    pub const REPO: &str = env!("CARGO_PKG_REPOSITORY");
}

pub mod config {
    pub const MAX_RECENT: usize = 5;
    pub const COMPRESS_BUFFER: usize = 1024 * 20;
    pub const SOCKET_BUFFER: usize = 1024 * 16; // needs to be <= COMPRESS_BUFFER
    pub const LOG_BUFFER: usize = 1024 * 9;
    pub const DEFAULT_SEQUENCE: i16 = 100;
}

pub struct Paths {
    pub app: PathBuf,
    pub base: PathBuf,
    pub logs: PathBuf,
    pub plugins: PathBuf,
    pub plugin_states: PathBuf,
    pub preferences: PathBuf,
    pub scripts: PathBuf,
    pub sounds: PathBuf,
    pub worlds: PathBuf,
}

impl Paths {
    pub fn new(settings: &RSettings) -> io::Result<Self> {
        let app = env::current_exe()?;
        let base = app.parent().unwrap().to_path_buf();
        let plugins = base.join("plugins");
        Ok(Self {
            worlds: base.join("words"),
            sounds: base.join("sounds"),
            scripts: base.join("scripts"),
            plugin_states: plugins.join("state"),
            plugins,
            preferences: settings.path(),
            logs: base.join("logs"),
            base,
            app,
        })
    }

    pub fn ensure(&self) -> io::Result<()> {
        for &path in &[
            &self.base,
            &self.logs,
            &self.plugins,
            &self.plugin_states,
            &self.scripts,
            &self.sounds,
            &self.worlds,
        ] {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}
