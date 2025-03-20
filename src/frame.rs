use std::cell::RefCell;

use ratzilla::ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Tabs, Widget},
    Frame,
};
use sigmatui::TAB_TITLES;

use crate::assets::BANNER;

#[derive(Default)]
pub struct AppFrame {}

impl AppFrame {
    pub fn render(&self, frame: &mut Frame, selected_tab: &u8) {
        self.title(&frame.area(), frame.buffer_mut());
        self.tab_bar(&frame.area(), frame.buffer_mut(), selected_tab);
        self.nav_controls(&frame.area(), frame.buffer_mut());
    }

    fn title(&self, area: &Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(*area, buf);

        Paragraph::new(BANNER).centered().render(*area, buf);
    }

    fn tab_bar(&self, area: &Rect, buf: &mut Buffer, selected_tab: &u8) {
        let tab_length = TAB_TITLES.join(" ").len() + 6;

        let tab_area = Rect::new(1, 0, tab_length as u16, 1);

        Tabs::new(TAB_TITLES.to_vec())
            .style(Style::new().fg(Color::DarkGray))
            .highlight_style(Style::new().fg(Color::White))
            .divider("|")
            .padding(" ", " ")
            .select(*selected_tab as usize)
            .render(tab_area, buf);
    }

    fn nav_controls(&self, area: &Rect, buf: &mut Buffer) {
        let controls = "| <- Previous Tab | -> Next Tab |";

        let nav_area = Rect::new(
            (area.width / 2) - (controls.len() as u16 / 2),
            area.height - 1,
            controls.len() as u16,
            1,
        );

        Line::from(controls).centered().render(nav_area, buf);
    }
}
