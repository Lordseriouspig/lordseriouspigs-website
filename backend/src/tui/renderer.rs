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
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget},
};
use strum::IntoEnumIterator;

use crate::app::models::state::{App, SelectedTab};
use crate::tui::themes::catppuccin::to_ratatui;
use crate::tui::views::*;

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        let background = Block::default().style(Style::default().bg(Color::Rgb(
            themes::catppuccin::BACKGROUND_PANE.rgb.r,
            themes::catppuccin::BACKGROUND_PANE.rgb.g,
            themes::catppuccin::BACKGROUND_PANE.rgb.b,
        )));

        background.render(area, buf);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        match self.selected_tab {
            SelectedTab::Home => {
                home::render(self.selected_tab.block(), inner_area, buf, &self.api_state)
            }
            SelectedTab::About => self.selected_tab.coming_soon(inner_area, buf),
            SelectedTab::AboutThisSite => {
                about_this_site::render(self.selected_tab.block(), inner_area, buf)
            }
            SelectedTab::Projects => self.selected_tab.coming_soon(inner_area, buf),
            SelectedTab::Contact => self.selected_tab.coming_soon(inner_area, buf),
        }
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(SelectedTab::title);
        let highlight_style = Style::default()
            .fg(Color::Rgb(
                themes::catppuccin::ON_ACCENT.rgb.r,
                themes::catppuccin::ON_ACCENT.rgb.g,
                themes::catppuccin::ON_ACCENT.rgb.b,
            ))
            .bg(Color::Rgb(
                themes::catppuccin::ACTIVE_BORDER.rgb.r,
                themes::catppuccin::ACTIVE_BORDER.rgb.g,
                themes::catppuccin::ACTIVE_BORDER.rgb.b,
            ));
        let selected_tab_index = self.selected_tab as usize;
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    "Lachlan's Website".bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl SelectedTab {
    /// Return tab's name as a styled `Line`
    fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(to_ratatui(themes::catppuccin::ON_ACCENT))
            .bg(to_ratatui(themes::catppuccin::INACTIVE_BORDER))
            .into()
    }

    fn coming_soon(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Coming Soon")
            .block(self.block())
            .render(area, buf);
    }

    /// A block surrounding the tab's content
    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(to_ratatui(themes::catppuccin::ACTIVE_BORDER))
    }
}
