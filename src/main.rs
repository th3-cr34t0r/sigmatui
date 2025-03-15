use assets::*;
mod assets;

use std::{cell::RefCell, io, rc::Rc};

use ratzilla::ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState, Paragraph, StatefulWidget},
    Frame, Terminal,
};

use ratzilla::{event::KeyCode, DomBackend, WebRenderer};

fn main() -> io::Result<()> {
    let tab_state = Rc::new(RefCell::new(Tab::Info));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut info_tab = InfoTab::new();
    let mut miner_tab = MinerTab::new();

    terminal.on_key_event({
        let tab_state_cloned = tab_state.clone();
        move |key_event| match key_event.code {
            KeyCode::Right => {
                let mut tab_state = tab_state_cloned.borrow_mut();
                *tab_state = Tab::Miner;
            }
            KeyCode::Left => {
                let mut tab_state = tab_state_cloned.borrow_mut();
                *tab_state = Tab::Info;
            }
            _ => {}
        }
    });

    terminal.draw_web(move |f| {
        let tab_state = tab_state.borrow();

        f.render_widget(
            Paragraph::new("").block(
                Block::bordered().border_type(ratzilla::ratatui::widgets::BorderType::Rounded),
            ),
            f.area(),
        );

        f.render_widget(Paragraph::new(BANNER).centered(), f.area());

        match *tab_state {
            Tab::Info => info_tab.render(f),
            Tab::Miner => miner_tab.render(f),
        }
    });

    Ok(())
}

enum Tab {
    Info,
    Miner,
}

struct InfoTab {}

impl InfoTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame) {
        self.top_addresses(f);
    }

    fn top_addresses(&mut self, f: &mut Frame) {
        let widget = List::new(vec![
            ListItem::new("addr_1"),
            ListItem::new("addr_2"),
            ListItem::new("addr_2"),
        ])
        .block(Block::bordered().title_top("Top Addresses"));
        let area = Rect::new(3, 6, (f.area().width as f32 * 0.33) as u16, 10);

        f.render_widget(widget, area);
    }
}

struct MinerTab {}

impl MinerTab {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(&mut self, f: &mut Frame) {
        self.miner_address(f);
    }
    fn miner_address(&mut self, f: &mut Frame) {
        let widget = Paragraph::new("Miner Address")
            .block(Block::bordered())
            .centered();

        let area = Rect::new(3, 5, 51, 3);

        f.render_widget(widget, area);
    }
}
