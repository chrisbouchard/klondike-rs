use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Settings {
    pub display: DisplaySettings,
    pub game: GameSettings,
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
