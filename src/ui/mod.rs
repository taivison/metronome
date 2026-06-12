mod widgets;
use ratatui::{
    buffer::Buffer,
    layout::{Direction, Layout, Offset, Rect, Spacing},
    macros::{constraint, constraints},
    style::{Style, palette::tailwind},
    widgets::{Block, List, Table, Widget},
};

use crate::app::App;
use widgets::*;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // Set Background
        Block::new()
            .style(Style::new().bg(tailwind::SLATE.c800))
            .render(area, buf);

        let area = area.centered(constraint!(==50), constraint!(==30));

        let main = Block::new().style(Style::new().bg(tailwind::SLATE.c900));

        let inner = main.inner(area);

        let shadow_area = area.offset(Offset::new(1, 1));
        Block::default()
            .style(Style::new().bg(tailwind::GRAY.c950))
            .render(shadow_area, buf);

        main.render(area, buf);

        let [body_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints![==50%, ==50%])
            .spacing(Spacing::Space(1))
            .areas(inner);

        let body = TitleBlock::new(" METRONOME ");
        let body_inner = body.inner(body_area);
        body.render(body_area, buf);

        let footer = TitleBlock::new(" SHORTCUTS ");
        let footer_inner = footer.inner(footer_area);
        footer.render(footer_area, buf);

        self.render_body(body_inner, buf);
        self.render_footer(footer_inner, buf);
    }
}

impl App {
    fn render_body(&self, area: Rect, buf: &mut Buffer) {
        let [stats_player, volume] = Layout::horizontal(constraints!(*=1, ==3))
            .spacing(1)
            .areas(area);

        let [stats, player] = Layout::vertical(constraints![==6, *=1])
            .spacing(1)
            .areas(stats_player);

        let [status, flags] = Layout::horizontal(constraints![==20, *=1]).areas(stats);

        let configs = self.config();

        Table::new(
            [
                StatusRow::new("BPM", configs.bpm, tailwind::ORANGE.c600),
                StatusRow::new("BEATS", configs.beats as u16, tailwind::EMERALD.c800),
                StatusRow::new(
                    "SUBDVISIONS",
                    configs.subdivisions as u16,
                    tailwind::SKY.c400,
                ),
            ],
            constraints![==12, *=1],
        )
        .column_spacing(1)
        .render(status, buf);

        MetronomePlayer::new(configs.beats, self.current_beat()).render(player, buf);

        VolumeBar::new(configs.volume).render(volume, buf);

        Table::new(
            [
                Flag::new(
                    configs.playing,
                    "STATUS",
                    tailwind::RED.c500,
                    tailwind::LIME.c400,
                    "⏹",
                    "▶",
                ),
                Flag::new(
                    configs.accent,
                    "ACCENT",
                    tailwind::ZINC.c600,
                    tailwind::YELLOW.c300,
                    "✗",
                    "✓",
                ),
            ],
            constraints![==7, *=1],
        )
        .column_spacing(1)
        .render(flags, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        List::new([
            "+/-  INCREASE/DECREASE VOLUME",
            "↑/↓  INCREASE/DECREASE BPM",
            "←/→  INCREASE/DECREASE BEATS",
            "S    CHANGE SUBDIVISIONS",
            "␣    PLAY/STOP",
            "A    TOGGLE ACCENT",
            "Q    QUIT",
        ])
        .render(area, buf);
    }
}
