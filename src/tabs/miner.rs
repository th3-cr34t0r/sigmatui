use std::{borrow::BorrowMut, cell::RefCell};

use ratzilla::ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::Line,
    widgets::{
        Axis, Block, BorderType, Chart, Clear, Dataset, GraphType, Paragraph, Row, Table, Widget,
    },
    Frame,
};

#[derive(Default)]
pub struct Miner {
    pub popup: bool,
    pub address: RefCell<String>,
}

impl Miner {
    pub fn render(&self, frame: &mut Frame) {
        let area = Rect::new(1, 6, frame.area().width - 2, frame.area().height - 7);

        let [top_area, bottom_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area);
        let [workers_area, hashrate_chart_area] =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Fill(1)]).areas(top_area);

        self.workers(&workers_area, frame.buffer_mut());
        self.hashrate_chart(&hashrate_chart_area, frame.buffer_mut());

        self.popup_address_input(&area, frame.buffer_mut());
    }

    pub fn char_to_insert(&self, char: char) {
        let mut address = self.address.borrow_mut();

        let index = address.len();

        address.insert(index, char);
    }

    fn workers(&self, area: &Rect, buf: &mut Buffer) {
        let rows = [
            Row::new(vec!["worker_1", "1225.6 Mh/s"]),
            Row::new(vec!["worker_2", "468.5 Mh/s"]),
            Row::new(vec!["worker_3", "856.5 Mh/s"]),
        ];

        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

        Table::new(rows, widths)
            .header(Row::new(vec!["Worker", "Hashrate"]).on_red())
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top("Address Workers"),
            )
            .render(*area, buf);
    }

    fn hashrate_chart(&self, area: &Rect, buf: &mut Buffer) {
        let data = vec![
            (0.0, 1.1),
            (1.0, 1.6),
            (2.0, 1.6),
            (3.0, 1.6),
            (4.0, 1.1),
            (5.0, 1.2),
            (6.0, 1.3),
            (7.0, 1.4),
            (8.0, 1.3),
            (9.0, 1.5),
            (10.0, 1.7),
            (11.0, 1.5),
            (12.0, 1.3),
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
                    .bounds([1.0, 2.0])
                    .labels(["1.0".bold(), "1.5".into(), "2.0".bold()])
                    .title("Gh/s"),
            )
            .render(*area, buf);
    }

    fn popup_address_input(&self, area: &Rect, buf: &mut Buffer) {
        if self.popup {
            let popup_area: [Rect; 3] = Layout::vertical([
                Constraint::Percentage(10),
                Constraint::Percentage(20),
                Constraint::Fill(1),
            ])
            .areas(*area);

            let [_, popup_area, _] = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Percentage(70),
                Constraint::Fill(1),
            ])
            .areas(popup_area[1]);

            Clear.render(popup_area, buf);
            Paragraph::new(self.address.borrow().as_str())
                .block(
                    Block::bordered()
                        .title_top("Input Miner Address")
                        .title_bottom(Line::from("| [P] Paste | [Enter] Search |").centered()),
                )
                .on_red()
                .centered()
                .render(popup_area, buf);
        }
    }
}
