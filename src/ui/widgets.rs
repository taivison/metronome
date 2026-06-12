use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{
        Color, Style, Styled,
        palette::tailwind::{self},
    },
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, Block, BorderType, Padding, Paragraph, Row, Widget},
};

use itertools::Itertools;

const BORDER_COLOR: Color = tailwind::SLATE.c600;

#[repr(transparent)]
pub struct TitleBlock<'a> {
    block: Block<'a>,
}

impl<'a> TitleBlock<'a> {
    pub fn new(title: &'static str) -> Self {
        Self {
            block: Block::bordered()
                .title(title)
                .title_alignment(Alignment::Center)
                .title_style(Style::new().bold().fg(BORDER_COLOR))
                .border_type(BorderType::Rounded)
                .border_style(Style::new().bold().fg(BORDER_COLOR))
                .padding(Padding::symmetric(2, 1)),
        }
    }

    pub fn inner(&self, area: Rect) -> Rect {
        self.block.inner(area)
    }
}

impl Widget for TitleBlock<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        self.block.render(area, buf);
    }
}

pub struct VolumeBar {
    volume: u8,
}

impl VolumeBar {
    #[must_use]
    pub fn new(volume: u8) -> Self {
        Self { volume }
    }
}

impl Widget for VolumeBar {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let colors = [
            tailwind::LIME.c800,
            tailwind::LIME.c700,
            tailwind::LIME.c600,
            tailwind::LIME.c500,
        ];

        let index = (self.volume.saturating_sub(1) / 25) as usize;

        BarChart::vertical([Bar::new(0), Bar::new(self.volume as u64), Bar::new(0)])
            .block(
                Block::new()
                    .title_top("VOL")
                    .title_alignment(Alignment::Center)
                    .title_bottom(
                        format!("{:03}", self.volume).set_style(Style::new().fg(colors[index])),
                    ),
            )
            .bar_gap(0)
            .bar_style(Style::new().fg(colors[index]))
            .bar_width(1)
            .max(100)
            .render(area, buf);
    }
}

pub struct MetronomePlayer {
    total_beats: u8,
    current_beat: u8,
}

impl MetronomePlayer {
    #[must_use]
    pub fn new(total_beats: u8, current_beat: u8) -> Self {
        Self {
            total_beats,
            current_beat,
        }
    }
}

impl Widget for MetronomePlayer {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let circles = (1..=self.total_beats)
            .map(|i| if i == self.current_beat { "⬤" } else { "◯" })
            .intersperse("  ");

        let arrow = (1..=self.total_beats)
            .map(|i| if i == self.current_beat { "▲" } else { " " })
            .intersperse("  ");

        Paragraph::new(Text::from_iter([
            Line::from_iter(circles),
            Line::from_iter(arrow),
        ]))
        .style(Style::new().fg(tailwind::INDIGO.c500).bold())
        .centered()
        .render(area, buf);
    }
}

pub struct StatusRow {
    header: &'static str,
    value: u16,
    color: Color,
}

impl StatusRow {
    #[must_use]
    pub fn new(header: &'static str, value: u16, color: Color) -> Self {
        Self {
            header,
            value,
            color,
        }
    }
}

impl From<StatusRow> for Row<'_> {
    fn from(value: StatusRow) -> Self {
        Row::new([
            Line::from_iter([Span::raw(value.header), Span::raw(":")]),
            Line::from(Span::raw(format!("{:3}", value.value))),
        ])
        .style(Style::new().fg(value.color))
        .bottom_margin(1)
    }
}

pub struct Flag {
    value: bool,
    header: &'static str,
    colors: [Color; 2],
    symbols: [&'static str; 2],
}

impl Flag {
    #[must_use]
    pub fn new(
        value: bool,
        header: &'static str,
        off_color: Color,
        on_color: Color,
        off_symbol: &'static str,
        on_symbol: &'static str,
    ) -> Self {
        Self {
            value,
            header,
            colors: [off_color, on_color],
            symbols: [off_symbol, on_symbol],
        }
    }
}

impl From<Flag> for Row<'_> {
    fn from(value: Flag) -> Self {
        let idx = value.value as usize;

        let color = value.colors[idx];
        let symbol = value.symbols[idx];

        Row::new([
            Line::from_iter([Span::raw(value.header), Span::raw(":")]),
            Line::from(Span::raw(symbol)),
        ])
        .style(Style::new().fg(color))
        .bottom_margin(1)
    }
}
