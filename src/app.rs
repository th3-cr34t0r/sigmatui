use crate::frame::AppFrame;
use crate::tabs::{home::Home, info::Info, miner::Miner};
use ratzilla::{
    event::KeyCode,
    ratatui::{layout::Rect, Frame, Terminal},
    DomBackend, WebRenderer,
};
use sigmatui::Tab;

pub struct App {
    pub tab_selected: u8,
    app_frame: AppFrame,
    home: Home,
    miner: Miner,
    info: u8,
}

impl App {
    pub fn new() -> Self {
        Self {
            tab_selected: 0,
            app_frame: AppFrame::new(),
            home: Home::new(),
            miner: Miner::new(),
            info: 0,
        }
    }

    pub fn run(&mut self, f: &mut Frame) {
        self.app_frame.render(f, &self.tab_selected);

        match Tab::new(&self.tab_selected) {
            Tab::Home => self.home.render(f),
            Tab::Miner => self.miner.render(f),
            Tab::Info => {}
        }
    }
}
