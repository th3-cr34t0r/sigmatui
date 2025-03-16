pub const TAB_LENGTH: u8 = 4;
pub const TAB_TITLES: [&str; TAB_LENGTH as usize] = ["Home", "Miner", "Info", "INFO"];
pub enum Tab {
    Home,
    Miner,
    Info,
}

impl Tab {
    pub fn new(tab_num_selected: &u8) -> Self {
        match *tab_num_selected {
            0 => Tab::Home,
            1 => Tab::Miner,
            2 => Tab::Info,

            _ => Tab::Info,
        }
    }
}
