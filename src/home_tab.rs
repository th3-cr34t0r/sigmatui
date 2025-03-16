use ratzilla::ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Chart, Dataset, List, ListItem, Paragraph},
    Frame,
};

pub struct HomeTab {}

impl HomeTab {
    pub fn new() -> Self {
        Self {}
    }
    ///Render the tab frame
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top_area, bottom_area] = vertical.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        let [top_addresses_area, pool_hashrate_area] = horizontal.areas(top_area);

        self.top_addresses(f, top_addresses_area);

        self.pool_hashrate(f, pool_hashrate_area);

        self.pool_info(f, bottom_area);
        // let (widget, area) = self.top_addresses(bottom_left);
        // f.render_widget(widget, area);

        // let (widget, area) = self.top_addresses(bottom_right);
        // f.render_widget(widget, area);
    }

    ///Provide logic for the top addresses
    fn top_addresses(&mut self, f: &mut Frame, area: Rect) {
        let widget = List::new(vec![
            ListItem::new("addr_1"),
            ListItem::new("addr_2"),
            ListItem::new("addr_2"),
        ])
        .block(Block::bordered().title_top("Top Addresses"));

        f.render_widget(widget, area);
    }
    fn pool_hashrate(&mut self, f: &mut Frame, area: Rect) {
        let dataset = vec![(1.5, 16.1), (1.6, 16.6), (1.5, 13.6), (1.4, 18.6)];

        let chart = Chart::new(vec![Dataset::default().data(&dataset)])
            .block(Block::bordered().title_top("Pool Hashrate"));

        f.render_widget(chart, area);
    }
    fn pool_info(&mut self, f: &mut Frame, area: Rect) {
        let widget = Paragraph::new("Pool Info")
            .block(Block::bordered())
            .alignment(Alignment::Center);

        f.render_widget(widget, area);
    }
}
