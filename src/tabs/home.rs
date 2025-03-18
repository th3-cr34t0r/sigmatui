use ratzilla::ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{Color, Stylize},
    symbols,
    widgets::{
        canvas::Label, Axis, Bar, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType,
        List, ListItem, Padding, Paragraph, Row, Table, Widget,
    },
    Frame,
};

pub struct Home {}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    ///Render home
    pub fn render(&mut self, f: &mut Frame) {
        let area = Rect::new(1, 6, f.area().width - 2, f.area().height - 7);
        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
        let [top_area, bottom_area] = vertical.areas(area);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]);

        let [top_addresses_area, pool_hashrate_area] = horizontal.areas(top_area);

        self.top_addresses(top_addresses_area, f.buffer_mut());

        self.pool_hashrate_chart(pool_hashrate_area, f.buffer_mut());

        self.pool_info(bottom_area, f.buffer_mut());
    }

    ///Provide logic for the top addresses
    fn top_addresses(&mut self, area: Rect, buf: &mut Buffer) {
        let rows = [
            Row::new(vec!["address_1", "12.2 Gh/s"]),
            Row::new(vec!["address_2", "11.5 Gh/s"]),
            Row::new(vec!["address_3", "hashrate"]),
            Row::new(vec!["address_4", "hashrate"]),
            Row::new(vec!["address_5", "hashrate"]),
            Row::new(vec!["address_6", "hashrate"]),
        ];

        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

        Table::new(rows, widths)
            .header(Row::new(vec!["Address", "Hashrate"]).on_red())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Top Miners"),
            )
            .render(area, buf);
    }

    ///Provide logic for pool hashrate section
    fn pool_hashrate_chart(&mut self, area: Rect, buf: &mut Buffer) {
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

        Chart::new(vec![dataset])
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
            )
            .render(area, buf);
    }

    ///Provide logic for pool info section
    fn pool_info(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("")
            .block(
                Block::bordered()
                    .border_set(symbols::border::ROUNDED)
                    .title_top("Pool Info"),
            )
            .render(area, buf);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

        let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let areas: [Rect; 3] = horizontal.areas(area);
        let [middle_up, middle_down] = vertical.areas(areas[1]);

        Gauge::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .padding(Padding::vertical(1))
                    .title("Current Block Effort"),
            )
            .ratio(15.0 / 200.0)
            .label(format!("{:.1}% / {:.1}%", 33, 200))
            .render(middle_up, buf);

        Gauge::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .padding(Padding::vertical(1))
                    .title("Last Block Effort"),
            )
            .ratio(110.0 / 200.0)
            .label(format!("{:.1}% / {:.1}%", 33, 200))
            .render(middle_down, buf);
    }
}
