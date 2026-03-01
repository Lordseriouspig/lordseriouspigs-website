// Copyright (C) 2026 Lordseriouspig
//
// This file is part of lordseriouspigs-website.
//
// lordseriouspigs-website is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// lordseriouspigs-website is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with lordseriouspigs-website.  If not, see <https://www.gnu.org/licenses/>.

use crate::tui::themes;
use std::vec;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};

const ASCII_ART_1: &str = r#" _    _      _ _       _    _____ _
| |  | |    | | |     | |  |_   _( )
| |__| | ___| | | ___ | |    | | |/ _ __ ___
|  __  |/ _ \ | |/ _ \| |    | |   | '_ ` _ \
| |  | |  __/ | | (_) |_|   _| |_  | | | | | |
|_|  |_|\___|_|_|\___/(_)  |_____| |_| |_| |_|"#;
const ASCII_ART_2: &str = r#"██╗      ██████╗ ██████╗ ██████╗ ███████╗███████╗██████╗ ██╗ ██████╗ ██╗   ██╗███████╗██████╗ ██╗ ██████╗
██║     ██╔═══██╗██╔══██╗██╔══██╗██╔════╝██╔════╝██╔══██╗██║██╔═══██╗██║   ██║██╔════╝██╔══██╗██║██╔════╝
██║     ██║   ██║██████╔╝██║  ██║███████╗█████╗  ██████╔╝██║██║   ██║██║   ██║███████╗██████╔╝██║██║  ███╗
██║     ██║   ██║██╔══██╗██║  ██║╚════██║██╔══╝  ██╔══██╗██║██║   ██║██║   ██║╚════██║██╔═══╝ ██║██║   ██║
███████╗╚██████╔╝██║  ██║██████╔╝███████║███████╗██║  ██║██║╚██████╔╝╚██████╔╝███████║██║     ██║╚██████╔╝
╚══════╝ ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝╚═╝  ╚═╝╚═╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝     ╚═╝ ╚═════╝"#;

pub fn render(border: Block<'static>, area: Rect, buf: &mut Buffer) {
    let border = border.padding(ratatui::widgets::Padding::horizontal(1));
    let inner_area = border.inner(area);
    border.render(area, buf);
    let block = Block::new().padding(ratatui::widgets::Padding::horizontal(1));
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(inner_area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Fill(1),
        ])
        .split(layout[0]);
    Paragraph::new(ASCII_ART_1)
        .block(block.clone())
        .render(inner[0], buf);
    Paragraph::new(ASCII_ART_2)
        .block(block)
        .style(Style::new().fg(Color::Rgb(
            themes::catppuccin::HEADING_TEXT.rgb.r,
            themes::catppuccin::HEADING_TEXT.rgb.g,
            themes::catppuccin::HEADING_TEXT.rgb.b,
        )))
        .render(inner[1], buf);
}
