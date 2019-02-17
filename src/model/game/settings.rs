#[derive(Debug)]
pub struct KlondikeGameSettings {
    pub draw_from_stock_len: usize,
    pub tableaux_len: usize,
    pub take_from_foundation: bool,
}

impl KlondikeGameSettings {
    pub fn foundation_indices(&self) -> impl Iterator<Item = usize> {
        0..4
    }

    pub fn tableaux_indices(&self) -> impl Iterator<Item = usize> {
        0..self.tableaux_len
    }
}

impl Default for KlondikeGameSettings {
    fn default() -> Self {
        KlondikeGameSettings {
            draw_from_stock_len: 3,
            tableaux_len: 7,
            take_from_foundation: true,
        }
    }
}
