use ratzilla::ratatui::{
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols,
    widgets::{
        canvas::Label, Axis, Block, BorderType, Borders, Chart, Dataset, GraphType, List, ListItem,
        Paragraph, Row, Table,
    },
    Frame,
};

pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    ///Render the tab frame
    pub fn render(&mut self, f: &mut Frame) {
        let area = Rect::new(1, 6, f.area().width - 2, f.area().height - 7);
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top_area, bottom_area] = vertical.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        let [top_addresses_area, pool_hashrate_area] = horizontal.areas(top_area);

        self.top_addresses(f, top_addresses_area);

        self.pool_hashrate(f, pool_hashrate_area);

        self.pool_info(f, bottom_area);
    }

    ///Provide logic for the top addresses
    fn top_addresses(&mut self, f: &mut Frame, area: Rect) {
        let rows = [
            Row::new(vec!["address_1", "12.2 Gh/s"]),
            Row::new(vec!["address_2", "11.5 Gh/s"]),
            Row::new(vec!["address_3", "hashrate"]),
            Row::new(vec!["address_4", "hashrate"]),
            Row::new(vec!["address_5", "hashrate"]),
            Row::new(vec!["address_6", "hashrate"]),
        ];

        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

        let table = Table::new(rows, widths)
            .header(Row::new(vec!["Address", "Hashrate"]).on_red())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Top Miners"),
            );

        f.render_widget(table, area);
    }

    ///Provide logic for pool hashrate section
    fn pool_hashrate(&mut self, f: &mut Frame, area: Rect) {
        let data = vec![
            (0.0, 16.1),
            (1.0, 16.6),
            (2.0, 13.6),
            (3.0, 18.6),
            (4.0, 16.1),
            (5.0, 16.6),
            (6.0, 13.6),
            (7.0, 18.6),
            (8.0, 16.1),
            (9.0, 16.6),
            (10.0, 13.6),
            (11.0, 18.6),
            (12.0, 16.1),
        ];

        let dataset = Dataset::default()
            .graph_type(GraphType::Line)
            .data(&data)
            .light_yellow();

        let chart = Chart::new(vec![dataset])
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Pool Hashrate"),
            )
            .x_axis(
                Axis::default()
                    .bounds([0.0, 12.0])
                    .labels(["0.0".bold(), "6.0".into(), "12.0".bold()])
                    .title("Hours"),
            )
            .y_axis(
                Axis::default()
                    .bounds([13.0, 19.0])
                    .labels(["13.0".bold(), "15.0".into(), "19.0".bold()])
                    .title("Gh/s"),
            );

        f.render_widget(chart, area);
    }

    ///Provide logic for pool info section
    fn pool_info(&mut self, f: &mut Frame, area: Rect) {
        let widget = Paragraph::new("").block(
            Block::bordered()
                .border_set(symbols::border::ROUNDED)
                .title_top("Pool Info"),
        );
        f.render_widget(widget, area);
    }
}
