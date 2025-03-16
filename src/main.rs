use assets::*;
use sigmatui::{Tab, TAB_LENGTH};

mod assets;
mod home_tab;
mod main_frame;
mod miner_tab;

use std::{cell::RefCell, io, rc::Rc};

use crate::home_tab::HomeTab;
use crate::main_frame::MainFrame;
use crate::miner_tab::MinerTab;

use ratzilla::{
    event::KeyCode,
    ratatui::{layout::Rect, Terminal},
    DomBackend, WebRenderer,
};

fn main() -> io::Result<()> {
    let tab_num_selected: Rc<RefCell<u8>> = Rc::new(RefCell::new(0));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut main_frame = MainFrame::new();

    let mut home_tab = HomeTab::new();
    let mut miner_tab = MinerTab::new();

    terminal.on_key_event({
        let tab_num_selected_cloned = tab_num_selected.clone();
        move |key_event| match key_event.code {
            KeyCode::Left => {
                let mut tab_num_selected = tab_num_selected_cloned.borrow_mut();
                if *tab_num_selected != 0 {
                    *tab_num_selected -= 1;
                }
            }
            KeyCode::Right => {
                let mut tab_num_selected = tab_num_selected_cloned.borrow_mut();
                if *tab_num_selected != (TAB_LENGTH - 1) {
                    *tab_num_selected += 1;
                }
            }
            _ => {}
        }
    });

    terminal.draw_web(move |f| {
        let tab_num_selected = tab_num_selected.borrow();

        main_frame.render(f, &tab_num_selected);

        let tab_selected = Tab::new(&tab_num_selected);

        match tab_selected {
            Tab::Home => {
                //prepare the area for indented widgets
                let area = Rect::new(1, 6, f.area().width - 2, f.area().height - 7);
                home_tab.render(f, area);
            }
            Tab::Miner => miner_tab.render(f),
            Tab::Info => {}
        }
    });

    Ok(())
}
