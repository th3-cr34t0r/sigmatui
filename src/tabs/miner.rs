use std::{borrow::BorrowMut, cell::RefCell};

use ratzilla::ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Chart, Clear, Paragraph, Widget},
    Frame,
};

#[derive(Default)]
pub struct Miner {
    pub popup: bool,
    pub address: RefCell<String>,
}

impl Miner {
    pub fn render(&self, frame: &mut Frame) {
        let area = Rect::new(1, 6, frame.area().width - 2, frame.area().height - 7);

        self.miner(&area, frame.buffer_mut());

        self.popup_address_input(&area, frame.buffer_mut());
    }

    pub fn char_to_insert(&self, char: char) {
        let mut address = self.address.borrow_mut();

        let index = address.len();

        address.insert(index, char);
    }

    fn miner(&self, area: &Rect, buf: &mut Buffer) {
        Paragraph::new("")
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top(self.address.borrow().as_str()),
            )
            .render(*area, buf);
    }

    fn popup_address_input(&self, area: &Rect, buf: &mut Buffer) {
        if self.popup {
            let popup_area: [Rect; 3] = Layout::vertical([
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Fill(1),
            ])
            .areas(*area);

            let [_, popup_area, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Percentage(70),
                Constraint::Fill(1),
            ])
            .areas(popup_area[1]);

            Clear.render(popup_area, buf);
            Paragraph::new(self.address.borrow().as_str())
                .block(
                    Block::bordered()
                        .title_top("Input Miner Address")
                        .title_bottom(Line::from("| [P] Paste | [Enter] Search |").centered()),
                )
                .on_red()
                .centered()
                .render(popup_area, buf);
        }
    }
}
