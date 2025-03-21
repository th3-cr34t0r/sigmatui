use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::frame::AppFrame;
use crate::tabs::info::Info;
use crate::tabs::{home::Home, miner::Miner};
use clipboard::{ClipboardContext, ClipboardProvider};
use ratzilla::event::KeyEvent;
use ratzilla::{event::KeyCode, ratatui::Frame};
use sigmatui::{Tab, TAB_LENGTH};

#[derive(Default)]
pub struct App {
    selected_tab: RefCell<u8>,
    app_frame: AppFrame,
    home: Home,
    miner: RefCell<Miner>,
    info: Info,
}

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let tab_selected = self.selected_tab.borrow();
        self.app_frame.render(frame, &tab_selected);

        match Tab::new(&tab_selected) {
            Tab::Home => self.home.render(frame),
            Tab::Miner => self.miner.borrow().render(frame),
            Tab::Info => self.info.render(),
        }
    }
    pub fn handle_events(&self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => {
                let mut miner = self.miner.borrow_mut();
                miner.popup = false;

                let mut selected_tab = self.selected_tab.borrow_mut();
                if *selected_tab != 0 {
                    *selected_tab -= 1;
                }
            }
            KeyCode::Right => {
                let mut miner = self.miner.borrow_mut();
                miner.popup = false;

                let mut selected_tab = self.selected_tab.borrow_mut();
                if *selected_tab != (TAB_LENGTH - 1) {
                    *selected_tab += 1;
                }
            }
            KeyCode::Char(char) => {
                if char == 's' {
                    let selected_tab = self.selected_tab.borrow();
                    if let Tab::Miner = Tab::new(&selected_tab) {
                        let mut miner = self.miner.borrow_mut();
                        miner.popup = !miner.popup;
                    }
                } else if char == 'p' {
                    let selected_tab = self.selected_tab.borrow();
                    if let Tab::Miner = Tab::new(&selected_tab) {
                        let mut miner = self.miner.borrow_mut();
                        if miner.popup {
                            let mut clipboard_content: ClipboardContext =
                                ClipboardProvider::new().unwrap();

                            miner.address = clipboard_content.get_contents().unwrap();
                            println!("{:?}", miner.address);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
