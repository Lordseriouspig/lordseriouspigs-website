// Copyright (C) 2026 Lordseriouspig
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    Terminal,
};

use crate::app::models::state::{App, AppState};

impl App {
    pub fn run<B: ratatui::backend::Backend>(mut self, mut terminal: Terminal<B>) -> Result<()> {
        while self.state == AppState::Running {
            self.tick(&mut terminal)?;
        }
        Ok(())
    }

    pub fn tick<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        if let Err(err) = terminal.draw(|frame| {
            frame.render_widget(&*self, frame.area())
        }) {
            eprintln!("draw failed: {err}");
            self.quit();
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_input(Event::Key(key));
            }
        }
        Ok(())
    }

    pub fn handle_input(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}