/*
 * Copyright (C) 2026 Lordseriouspig
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::tui::themes;
use crate::tui::themes::catppuccin::to_ratatui;
use ratatui::layout::Rect;
use ratatui::prelude::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn default_inner(border: Block<'static>, area: Rect) -> (Block<'static>, Rect) {
    let border = border.padding(ratatui::widgets::Padding::horizontal(1));
    let inner_area = border.inner(area);
    (border, inner_area)
}

pub fn default_block() -> Block<'static> {
    Block::new().padding(ratatui::widgets::Padding::horizontal(1))
}

pub fn default_heading<'a>(p: &'a str, block: Block<'a>) -> Paragraph<'a> {
    Paragraph::new(p)
        .block(block)
        .style(Style::new().fg(to_ratatui(themes::catppuccin::HEADING_TEXT)))
}

pub fn default_paragraph<'a>(p: Vec<Line<'a>>, block: Block<'a>) -> Paragraph<'a> {
    Paragraph::new(p)
        .block(block)
        .style(Style::new().fg(to_ratatui(themes::catppuccin::BODY_TEXT)))
        .wrap(Wrap { trim: false })
}

pub fn border_paragraph<'a>(p: Vec<Line<'a>>, block: Block<'a>) -> Paragraph<'a> {
    Paragraph::new(p)
        .block(block.borders(Borders::ALL))
        .style(Style::new().fg(to_ratatui(themes::catppuccin::BODY_TEXT)))
        .wrap(Wrap { trim: false })
}
