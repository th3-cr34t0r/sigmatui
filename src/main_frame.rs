use ratzilla::ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Tabs},
    Frame,
};
use sigmatui::TAB_TITLES;

use crate::BANNER;

pub struct MainFrame {}

impl MainFrame {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, f: &mut Frame, selected_tab: &u8) {
        self.title(f);
        self.tab_bar(f, selected_tab);
        self.nav_controls(f);
    }

    fn title(&mut self, f: &mut Frame) {
        let border = Paragraph::new("")
            .block(Block::bordered().border_type(ratzilla::ratatui::widgets::BorderType::Rounded));
        f.render_widget(border, f.area());

        let title = Paragraph::new(BANNER).centered();
        f.render_widget(title, f.area());
    }
    fn nav_controls(&mut self, f: &mut Frame) {
        let controls = "| <- Previous Tab | -> Next Tab |";
        let nav = Paragraph::new(controls).centered();

        let area = Rect::new(
            (f.area().width / 2) - (controls.len() as u16 / 2),
            f.area().height - 1,
            controls.len() as u16,
            1,
        );

        f.render_widget(nav, area);
    }

    fn tab_bar(&mut self, f: &mut Frame, selected_tab: &u8) {
        let tab_bar = Tabs::new(TAB_TITLES.to_vec()).highlight_style(Style::new().on_green());

        let tab_bar_selected = tab_bar.select(*selected_tab as usize);

        let tab_area = Rect::new(1, 1, (f.area().width as f32 * 0.3) as u16, 1);

        f.render_widget(tab_bar_selected, tab_area);
    }
}
