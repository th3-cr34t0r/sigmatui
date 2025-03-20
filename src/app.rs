use std::cell::RefCell;

use crate::frame::AppFrame;
use crate::tabs::info::Info;
use crate::tabs::{home::Home, miner::Miner};
use ratzilla::event::KeyEvent;
use ratzilla::{event::KeyCode, ratatui::Frame};
use sigmatui::{Tab, TAB_LENGTH};

#[derive(Default)]
pub struct App {
    pub tab_selected: RefCell<u8>,
    app_frame: AppFrame,
    home: Home,
    miner: Miner,
    info: Info,
}

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let tab_selected = self.tab_selected.borrow();
        self.app_frame.render(frame, &tab_selected);

        match Tab::new(&tab_selected) {
            Tab::Home => self.home.render(frame),
            Tab::Miner => self.miner.render(frame),
            Tab::Info => self.info.render(),
        }
    }
    pub fn handle_events(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => {
                let mut tab_selected = self.tab_selected.borrow_mut();
                if *tab_selected != 0 {
                    *tab_selected -= 1;
                }
            }
            KeyCode::Right => {
                let mut tab_selected = self.tab_selected.borrow_mut();
                if *tab_selected != (TAB_LENGTH - 1) {
                    *tab_selected += 1;
                }
            }
            _ => {}
        }
    }
}
