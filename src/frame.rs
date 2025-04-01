use ratzilla::ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Tabs, Widget},
    Frame,
};
use sigmatui::{Tab, TAB_TITLES};

use crate::assets::BANNER;

#[derive(Default)]
pub struct AppFrame {}

impl AppFrame {
    pub fn render(&self, frame: &mut Frame, selected_tab: &u8) {
        self.title(&frame.area(), frame.buffer_mut(), selected_tab);
        self.tab_bar(&frame.area(), frame.buffer_mut(), selected_tab);
    }

    fn title(&self, area: &Rect, buf: &mut Buffer, selected_tab: &u8) {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title_bottom(self.nav_controls(selected_tab).centered())
            .render(*area, buf);

        Paragraph::new(BANNER).centered().render(*area, buf);
    }

    fn tab_bar(&self, _area: &Rect, buf: &mut Buffer, selected_tab: &u8) {
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

    fn nav_controls(&self, selected_tab: &u8) -> Line<'_> {
        match Tab::new(selected_tab) {
            Tab::Home => Line::default().spans(vec![
                "| ".white(),
                "[<-]".light_yellow().bold(),
                " Previous Tab".white(),
                " | ".white(),
                "[->]".light_yellow().bold(),
                " Next Tab".white(),
                " |".white(),
            ]),

            Tab::Miner => Line::default().spans(vec![
                "| ".white(),
                "[<-]".light_yellow().bold(),
                " Previous Tab".white(),
                " | ".white(),
                "[->]".light_yellow().bold(),
                " Next Tab".white(),
                " | ".white(),
                "[S]".light_yellow().bold(),
                " Search Address".white(),
                " |".white(),
            ]),

            Tab::Info => Line::default().spans(vec![
                "| ".white(),
                "[<-]".light_yellow().bold(),
                " Previous Tab".white(),
                " | ".white(),
                "[->]".light_yellow().bold(),
                " Next Tab".white(),
                " |".white(),
            ]),
        }
    }
}
