use ratzilla::ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols::{self, Marker},
    text::{Line, Span},
    widgets::{
        Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType, Padding, Paragraph,
        Row, Table, Widget, Wrap,
    },
    Frame,
};

#[derive(Default)]
pub struct Home {}

impl Home {
    ///Render home
    pub fn render(&self, f: &mut Frame) {
        let area = Rect::new(1, 6, f.area().width - 2, f.area().height - 7);

        let [top_area, bottom_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);

        let [top_addresses_area, pool_hashrate_area] =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Fill(1)]).areas(top_area);

        let [pool_info_area, block_effort_area, how_to_connect_area] = Layout::horizontal([
            Constraint::Percentage(33),
            Constraint::Fill(1),
            Constraint::Percentage(33),
        ])
        .areas(bottom_area);

        let [block_effort_up, block_effort_down] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(block_effort_area);

        // Top half
        self.top_addresses(&top_addresses_area, f.buffer_mut());
        self.pool_hashrate_chart(&pool_hashrate_area, f.buffer_mut());

        // Bottom half
        self.pool_info(&pool_info_area, f.buffer_mut());
        self.current_effort_gauge(&35.0, &block_effort_up, f.buffer_mut());
        self.last_effort_gauge(&61.5, &block_effort_down, f.buffer_mut());
        self.how_to_connect(&how_to_connect_area, f.buffer_mut());
    }

    ///Provide logic for the top addresses
    fn top_addresses(&self, area: &Rect, buf: &mut Buffer) {
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
            .render(*area, buf);
    }

    ///Provide logic for pool hashrate section
    fn pool_hashrate_chart(&self, area: &Rect, buf: &mut Buffer) {
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
            .marker(Marker::Dot)
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
            .render(*area, buf);
    }

    ///Pool related info
    fn pool_info(&self, area: &Rect, buf: &mut Buffer) {
        Paragraph::new("")
            .block(
                Block::bordered()
                    .border_set(symbols::border::ROUNDED)
                    .title_top("Pool Info"),
            )
            .centered()
            .render(*area, buf);

        let [_left, horizontally_centered_area, _right] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(90),
            Constraint::Fill(1),
        ])
        .areas(*area);

        let [_top_margin, centered_area, _bottom_margin] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(90),
            Constraint::Fill(1),
        ])
        .areas(horizontally_centered_area);

        let [text_area, network_height_area, pool_hashrate_area, pool_miners_area] =
            Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .areas(centered_area);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .title_top("Motivation")
            .title_alignment(Alignment::Center)
            .fg(Color::White)
            .render(text_area, buf);

        let text_motivaton_area = Rect::new(
            text_area.x,
            text_area.y + (text_area.height / 3),
            text_area.width - 2,
            text_area.height - 2,
        );

        let motivation_text = Line::default().spans(vec!["Sigmanauts pool is a DAO-driven, community-run mining pool dedicated to supporting the Ergo ecosystem with extra storage rent (de-murage) rewards.".light_yellow().bold()]);

        Paragraph::new(motivation_text)
            .centered()
            .wrap(Wrap { trim: true })
            .render(text_motivaton_area, buf);

        let network_hashrate_info =
            Line::default().spans(vec![format!("{} Th/s", 6.52).light_yellow().bold()]);
        Paragraph::new(network_hashrate_info)
            .block(
                Block::new()
                    .title_top("Network Hashrate")
                    .title_alignment(Alignment::Center),
            )
            .centered()
            .render(network_height_area, buf);

        let pool_hashrate_info =
            Line::default().spans(vec![format!("{} Gh/s", 66.52).light_yellow().bold()]);
        Paragraph::new(pool_hashrate_info)
            .block(
                Block::new()
                    .title_top("Pool Hashrate")
                    .title_alignment(Alignment::Center),
            )
            .centered()
            .render(pool_hashrate_area, buf);

        let pool_miners_info = Line::default().spans(vec![format!("{}", 65).light_yellow().bold()]);
        Paragraph::new(pool_miners_info)
            .block(
                Block::new()
                    .title_top("Pool Miners")
                    .title_alignment(Alignment::Center),
            )
            .centered()
            .render(pool_miners_area, buf);
    }

    ///Block title
    fn title_block(&self, title: &'static str, color: Color) -> Block {
        let title = Line::from(title);
        Block::bordered()
            .title_top(title)
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .fg(color)
    }

    ///Current block effort gauge
    fn current_effort_gauge(&self, progress: &f64, area: &Rect, buf: &mut Buffer) {
        let title = self.title_block("Current Block Effort", Color::White);

        let label = Span::styled(
            format!("{:.1}/100", *progress),
            Style::new().italic().bold(),
        );

        Gauge::default()
            .block(title)
            .gauge_style(Style::new().fg(Color::LightRed).bg(Color::Red))
            .ratio(*progress / 100.0)
            .label(label)
            .render(*area, buf);
    }
    ///Last Block Effort gauge
    fn last_effort_gauge(&self, progress: &f64, area: &Rect, buf: &mut Buffer) {
        let title = self.title_block("Last Block Effort", Color::White);

        let label = Span::styled(format!("{:.1}/100", 66.4), Style::new().italic().bold());

        Gauge::default()
            .block(title)
            .gauge_style(Style::new().fg(Color::LightCyan).bg(Color::Cyan))
            .ratio(*progress / 100.0)
            .label(label)
            .render(*area, buf);
    }
    ///Block explaining how to connect
    fn how_to_connect(&self, area: &Rect, buf: &mut Buffer) {
        Paragraph::new("")
            .block(
                Block::bordered()
                    .border_set(symbols::border::ROUNDED)
                    .title_top("How To Connect"),
            )
            .centered()
            .render(*area, buf);

        let [_left, centered, _right] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(90),
            Constraint::Fill(1),
        ])
        .areas(*area);

        let [_fill_1, top, bottom, _fill_2] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Fill(1),
        ])
        .areas(centered);

        Block::new()
            .borders(Borders::TOP)
            .title_top("Under 10 Gh/s")
            .title_alignment(Alignment::Center)
            .fg(Color::White)
            .render(top, buf);

        let top_center_area = Rect::new(top.x, top.y + (top.height / 2), top.width, top.height);

        Paragraph::new(
            Line::default().spans(vec!["pool.ergo-sig-mining.net:3053".light_yellow().bold()]),
        )
        .centered()
        .render(top_center_area, buf);

        Block::new()
            .borders(Borders::TOP)
            .title_top("Over 10 Gh/s")
            .title_alignment(Alignment::Center)
            .fg(Color::White)
            .render(bottom, buf);

        let bottom_center_area = Rect::new(
            bottom.x,
            bottom.y + (bottom.height / 2),
            bottom.width,
            bottom.height,
        );

        Paragraph::new(
            Line::default().spans(vec!["pool.ergo-sig-mining.net:3055".light_yellow().bold()]),
        )
        .centered()
        .render(bottom_center_area, buf);
    }
}
