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
use crate::app::models::api::api_state::SharedApiState;
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone)]
pub struct App {
    pub state: AppState,
    pub selected_tab: SelectedTab,
    pub area_change: Option<ratatui::layout::Rect>,
    pub api_state: SharedApiState,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    NotReady,
    Running,
    Quitting,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Home")]
    Home,
    #[strum(to_string = "About")]
    About,
    #[strum(to_string = "About This Site")]
    AboutThisSite,
    #[strum(to_string = "Projects")]
    Projects,
    #[strum(to_string = "Contact")]
    Contact,
}
