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

#[derive(Debug)]
pub enum ClientInput {
    Key(ClientKey),
    Resize { cols: u16, rows: u16 },
}

#[derive(Debug)]
pub enum ClientKey {
    Char(char),
    Enter,
    Escape,
    ArrowLeft,
    ArrowRight,
    Tab,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum WireInput {
    Key { key: String },
    Resize { cols: u16, rows: u16 },
}
