use std::path;

use config::{Config, ConfigError, Environment, File, FileFormat};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

static QUALIFIER: &'static str = "net";
static ORGANIZATION: &'static str = "upflitinglemma";
static APPLICATION: &'static str = "klondike-rs";

static CONFIG_FILE: &'static str = "config.toml";

static ENV_PREFIX: &'static str = "klondike_";
static ENV_SEPARATOR: &'static str = "__";

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub game: GameSettings,
}

impl Settings {
    // TODO: Return a snafu-defined error type
    pub fn read_config() -> Result<Settings, ConfigError> {
        let mut config = Config::new();

        let config_path: Option<path::PathBuf> =
            ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).map(|project_dirs| {
                let mut path = project_dirs.config_dir().to_path_buf();
                path.push(CONFIG_FILE);
                info!("Looking for config file: {}", path.display());
                path
            });

        if let Some(config_path) = config_path {
            config.merge(
                File::from(config_path)
                    .format(FileFormat::Toml)
                    .required(false),
            )?;
        }

        config.merge(Environment::with_prefix(ENV_PREFIX).separator(ENV_SEPARATOR))?;

        config.try_into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DisplaySettings {
    pub color: bool,
    pub unicode: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {
            color: true,
            unicode: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GameSettings {
    pub draw_from_stock_len: usize,
    pub tableaux_len: u8,
    pub take_from_foundation: bool,
}

impl GameSettings {
    pub fn tableaux_indices(&self) -> impl Iterator<Item = u8> {
        0..self.tableaux_len
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            draw_from_stock_len: 3,
            tableaux_len: 7,
            take_from_foundation: true,
        }
    }
}
