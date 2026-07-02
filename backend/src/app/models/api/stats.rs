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
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct StatsConf {
    pub base_url: String,
    pub username: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatsResp {
    pub data: StatsData,
    pub trust_factor: StatsTF,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatsData {
    pub username: String,
    pub user_id: String,
    pub is_coding_activity_visible: bool,
    pub is_other_usage_visible: bool,
    pub status: String,
    pub start: String,
    pub end: String,
    pub range: String,
    pub human_readable_range: String,
    pub total_seconds: usize,
    pub daily_average: usize,
    pub human_readable_total: String,
    pub human_readable_daily_average: String,
    pub languages: Option<Vec<StatsObj>>,
    pub projects: Option<Vec<StatsObj>>,
    pub editors: Option<Vec<StatsObj>>,
    pub streak: usize,
    pub unique_total_seconds: Option<usize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatsObj {
    pub name: String,
    pub total_seconds: usize,
    pub text: String,
    pub hours: usize,
    pub minutes: usize,
    pub percent: f32,
    pub digital: String,
    pub color: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatsTF {
    pub trust_level: String,
    pub trust_value: usize,
}
