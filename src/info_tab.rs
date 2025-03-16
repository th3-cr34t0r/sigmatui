use ratzilla::ratatui::{
    layout::Rect,
    widgets::{Block, List, ListItem},
    Frame,
};

pub struct HomeTab {}

impl HomeTab {
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
