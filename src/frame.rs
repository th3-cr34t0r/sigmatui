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

pub struct AppFrame {}

impl AppFrame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame, selected_tab: &u8) {
        self.title(&f.area(), f.buffer_mut());
        self.tab_bar(&f.area(), f.buffer_mut(), selected_tab);
        self.nav_controls(&f.area(), f.buffer_mut());
    }

    fn title(&mut self, area: &Rect, buf: &mut Buffer) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(*area, buf);

        Paragraph::new(BANNER).centered().render(*area, buf);
    }

    fn tab_bar(&mut self, area: &Rect, buf: &mut Buffer, selected_tab: &u8) {
        let tab_area = Rect::new(1, 0, (area.width as f32 * 0.3) as u16, 1);

        Tabs::new(TAB_TITLES.to_vec())
            .highlight_style(Style::new().add_modifier(Modifier::UNDERLINED))
            .divider("|")
            .select(*selected_tab as usize)
            .render(tab_area, buf);
    }

    fn nav_controls(&mut self, area: &Rect, buf: &mut Buffer) {
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
