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