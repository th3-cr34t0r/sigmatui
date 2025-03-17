use ratzilla::ratatui::{
    layout::Rect,
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
pub struct Miner {}

impl Miner {
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
