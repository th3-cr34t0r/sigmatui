use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::frame::AppFrame;
use crate::tabs::info::Info;
use crate::tabs::{home::Home, miner::Miner};
use async_std::task;
use ratzilla::event::KeyEvent;
use ratzilla::{event::KeyCode, ratatui::Frame};
use sigmatui::{Tab, TAB_LENGTH};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(Default)]
enum InputMode {
    #[default]
    Normal,
    Input,
}

#[derive(Default)]
pub struct App {
    input_mode: RefCell<InputMode>,
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
                *self.input_mode.borrow_mut() = InputMode::Normal;

                let mut selected_tab = self.selected_tab.borrow_mut();
                if *selected_tab != 0 {
                    *selected_tab -= 1;
                }
            }
            KeyCode::Right => {
                let mut miner = self.miner.borrow_mut();
                miner.popup = false;
                *self.input_mode.borrow_mut() = InputMode::Normal;

                let mut selected_tab = self.selected_tab.borrow_mut();
                if *selected_tab != (TAB_LENGTH - 1) {
                    *selected_tab += 1;
                }
            }
            KeyCode::Char(char) => {
                let mut input_mode = self.input_mode.borrow_mut();
                match *input_mode {
                    InputMode::Normal => {
                        if char == 's' {
                            let selected_tab = self.selected_tab.borrow();

                            if let Tab::Miner = Tab::new(&selected_tab) {
                                let mut miner = self.miner.borrow_mut();

                                if !miner.popup {
                                    miner.popup = true;
                                    *input_mode = InputMode::Input;
                                } else {
                                    miner.popup = false;
                                    *input_mode = InputMode::Normal;
                                }
                            }
                        }
                    }
                    InputMode::Input => {
                        let selected_tab = self.selected_tab.borrow();
                        if let Tab::Miner = Tab::new(&selected_tab) {
                            let miner = self.miner.borrow_mut();
                            if miner.popup && char == 'p' {
                                // miner.char_to_insert(char);
                                let clipboard_content =
                                    task::block_on(async { get_clipboard_content().await });
                                // miner
                                //     .address
                                //     .borrow_mut()
                                //     .push_str(clipboard_content.ok().unwrap().as_str());

                                // if let Ok(content) = clipboard_content {
                                //     miner.address.borrow_mut().push_str(content.as_str());
                                // }
                            }
                        }
                    }
                }
            }
            KeyCode::Enter => {
                *self.input_mode.borrow_mut() = InputMode::Normal;
                let mut miner = self.miner.borrow_mut();
                if miner.popup {
                    miner.popup = false;
                }
            }
            _ => {}
        }
    }
}

#[wasm_bindgen]
pub async fn get_clipboard_content() -> Result<String, JsValue> {
    let window = window().expect("No global window exists");

    let clipboard = window.navigator().clipboard();

    let text = clipboard.read_text();

    let content = JsFuture::from(text).await.expect("").as_string().unwrap();

    Ok(content)
}
