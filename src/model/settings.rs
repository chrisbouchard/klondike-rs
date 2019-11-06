use config::{Config, ConfigError, Environment, File, FileFormat};
use directories::{ProjectDirs, UserDirs};
use serde::{Deserialize, Serialize};

static QUALIFIER: &str = "net";
static ORGANIZATION: &str = "upflitinglemma";
static APPLICATION: &str = "klondike-rs";

static CONFIG_FILE: &str = "config.toml";
static HOME_CONFIG_FILE: &str = ".klondike-rs.toml";

static ENV_PREFIX: &str = "klondike_";
static ENV_SEPARATOR: &str = "__";

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub game: GameSettings,
}

impl Settings {
    // TODO: Return a snafu-defined error type
    pub fn read_from_system() -> Result<Settings, ConfigError> {
        let mut config = Config::new();

        if let Some(user_dirs) = UserDirs::new() {
            let mut path = user_dirs.home_dir().to_path_buf();
            path.push(HOME_CONFIG_FILE);
            config.merge(File::from(path).format(FileFormat::Toml).required(false))?;
        }

        if let Some(project_dirs) = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION) {
            let mut path = project_dirs.config_dir().to_path_buf();
            path.push(CONFIG_FILE);
            config.merge(File::from(path).format(FileFormat::Toml).required(false))?;
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

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum DealerMode {
    AutoWin,
    InOrder,
    Random,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GameSettings {
    pub dealer: DealerMode,
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
            dealer: DealerMode::Random,
            draw_from_stock_len: 3,
            tableaux_len: 7,
            take_from_foundation: true,
        }
    }
}
