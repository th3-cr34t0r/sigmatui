use ratzilla::ratatui::{
    layout::Rect,
    widgets::{Block, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct Miner {}

impl Miner {
    pub fn render(&self, f: &mut Frame) {
        self.miner_address(f);
    }
    fn miner_address(&self, f: &mut Frame) {
        let widget = Paragraph::new("Miner Address")
            .block(Block::bordered())
            .centered();

        let area = Rect::new(3, 5, 51, 3);

        f.render_widget(widget, area);
    }
}
