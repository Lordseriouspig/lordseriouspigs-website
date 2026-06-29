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

use crate::app::models::client_event::{ClientInput, ClientKey};
use crate::app::models::sessions::session::Session;
use crate::app::models::state::{App, AppState};
use crate::middleware::websocket::writer::WsWriter;
use color_eyre::Result;
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

impl Session {
    pub async fn run(mut self) {
        println!("Running a new session");
        // spawn an instance of a custom backend for crossterm
        let writer = WsWriter {
            tx: self.output_tx.clone(),
            buffer: Vec::new(),
        };

        let backend = CrosstermBackend::new(writer);
        let mut terminal = Terminal::new(backend).unwrap();

        // Spawn the app
        let mut app = self.app;

        loop {
            if app.state == AppState::Quitting {
                break;
            }
            tokio::select! {
                Some(input) = self.input_rx.recv() => {
                    app.handle_input(input);
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(10)) => {
                    let _ = app.render(&mut terminal);
                }
            }
        }
    }
}

impl App {
    pub fn render<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        if let Err(err) = terminal.draw(|frame| frame.render_widget(&*self, self.area)) {
            eprintln!("draw failed: {err}");
            self.quit();
            return Ok(());
        }
        let _ = terminal.backend_mut().flush();
        Ok(())
    }

    pub fn handle_input(&mut self, input: ClientInput) {
        println!("Received input: {:?}", input);
        match input {
            ClientInput::Key(k) => self.handle_key(k),
            ClientInput::Resize { cols, rows } => self.handle_resize(cols, rows),
        }
    }

    pub fn handle_key(&mut self, key: ClientKey) {
        match key {
            ClientKey::Char('l') | ClientKey::ArrowRight => self.next_tab(),
            ClientKey::Char('h') | ClientKey::ArrowLeft => self.previous_tab(),
            ClientKey::Char('q') | ClientKey::Escape => self.quit(),
            _ => {}
        }
    }

    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        self.area = ratatui::layout::Rect::new(0, 0, cols, rows);
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
