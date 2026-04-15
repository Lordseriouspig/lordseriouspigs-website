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
    widgets::{Block, Paragraph, Widget, Wrap},
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

const BODY_TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed condimentum nulla bibendum enim fermentum, id viverra ligula posuere. Curabitur hendrerit eros sed finibus porttitor. Vestibulum nec sollicitudin turpis. Maecenas sed nisl turpis. Mauris porttitor nec nunc vitae commodo. Nunc eget nibh sit amet ipsum tempus maximus. Duis venenatis malesuada metus imperdiet sodales. Etiam eu facilisis lectus. Nulla dui nisi, lobortis quis fermentum non, ultricies nec diam. Sed tristique est et augue viverra, id porta ex dignissim. Curabitur finibus nisl id sem suscipit, ut rhoncus enim aliquam. Phasellus ac lorem sit amet massa bibendum posuere. In convallis efficitur cursus. Etiam justo sapien, bibendum ac arcu ut, accumsan mattis ipsum. Vestibulum a venenatis elit.";

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
            Constraint::Fill(2)
        ])
        .split(layout[0]);
    Paragraph::new(ASCII_ART_1)
        .block(block.clone())
        .render(inner[0], buf);
    Paragraph::new(ASCII_ART_2)
        .block(block.clone())
        .style(Style::new().fg(Color::Rgb(
            themes::catppuccin::HEADING_TEXT.rgb.r,
            themes::catppuccin::HEADING_TEXT.rgb.g,
            themes::catppuccin::HEADING_TEXT.rgb.b,
        )))
        .render(inner[1], buf);
    Paragraph::new(BODY_TEXT)
        .block(block)
        .style(Style::new().fg(Color::Rgb(
            themes::catppuccin::BODY_TEXT.rgb.r,
            themes::catppuccin::BODY_TEXT.rgb.g,
            themes::catppuccin::BODY_TEXT.rgb.b,
        )))
        .wrap(Wrap { trim: true })
        .render(inner[2], buf);
    // TODO: Hackatime stats w/ loading anim
    // TODO: Right column w/ links, stats, github stuff, slack api stuff, time, etc
}
