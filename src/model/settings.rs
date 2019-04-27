#[derive(Debug)]
pub struct Settings {
    pub draw_from_stock_len: usize,
    pub tableaux_len: u8,
    pub take_from_foundation: bool,
}

impl Settings {
    pub fn tableaux_indices(&self) -> impl Iterator<Item = u8> {
        0..self.tableaux_len
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            draw_from_stock_len: 3,
            tableaux_len: 7,
            take_from_foundation: true,
        }
    }
}
