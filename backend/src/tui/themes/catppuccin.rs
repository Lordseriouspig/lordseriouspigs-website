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

use catppuccin::{Color as ThemeColor, FlavorColors};
use ratatui::style::Color as RatatuiColor;

const fn modify_color(color: ThemeColor) -> ThemeColor {
    let mut modified = color;
    modified.hsl.l *= 0.94; // lightness * 0.94
    modified.hsl.s += 0.08; // saturation + 8%
    modified.hsl.h = (modified.hsl.h + 2.0) % 360.0; // hue + 2 degrees

    modified.rgb.r = (modified.rgb.r as f32 * 1.05).min(255.0) as u8;
    modified.rgb.g = (modified.rgb.g as f32 * 1.05).min(255.0) as u8;
    modified.rgb.b = (modified.rgb.b as f32 * 1.05).min(255.0) as u8;

    modified
}

pub const fn get_bright_colors() -> FlavorColors {
    let colors = COLORS;
    // there is probably a better way to do this
    FlavorColors {
        rosewater: modify_color(colors.rosewater),
        flamingo: modify_color(colors.flamingo),
        pink: modify_color(colors.pink),
        mauve: modify_color(colors.mauve),
        red: modify_color(colors.red),
        maroon: modify_color(colors.maroon),
        peach: modify_color(colors.peach),
        yellow: modify_color(colors.yellow),
        green: modify_color(colors.green),
        teal: modify_color(colors.teal),
        sky: modify_color(colors.sky),
        sapphire: modify_color(colors.sapphire),
        blue: modify_color(colors.blue),
        lavender: modify_color(colors.lavender),
        text: modify_color(colors.text),
        subtext1: modify_color(colors.subtext1),
        subtext0: modify_color(colors.subtext0),
        overlay2: modify_color(colors.overlay2),
        overlay1: modify_color(colors.overlay1),
        overlay0: modify_color(colors.overlay0),
        surface2: modify_color(colors.surface2),
        surface1: modify_color(colors.surface1),
        surface0: modify_color(colors.surface0),
        base: modify_color(colors.base),
        mantle: modify_color(colors.mantle),
        crust: modify_color(colors.crust),
    }
}
pub const COLORS: FlavorColors = catppuccin::PALETTE.mocha.colors;
pub const BRIGHT_COLORS: FlavorColors = get_bright_colors();

pub const BACKGROUND_PANE: ThemeColor = COLORS.base;
pub const SECONDARY_PANE_0: ThemeColor = COLORS.crust;
pub const SECONDARY_PANE_1: ThemeColor = COLORS.mantle;
pub const SURFACE_ELEMENT_0: ThemeColor = COLORS.surface0;
pub const SURFACE_ELEMENT_1: ThemeColor = COLORS.surface1;
pub const SURFACE_ELEMENT_2: ThemeColor = COLORS.surface2;
pub const OVERLAY_0: ThemeColor = COLORS.overlay0;
pub const OVERLAY_1: ThemeColor = COLORS.overlay1;
pub const OVERLAY_2: ThemeColor = COLORS.overlay2;

pub const BODY_TEXT: ThemeColor = COLORS.text;
pub const HEADING_TEXT: ThemeColor = COLORS.mauve;
pub const SUBTEXT_0: ThemeColor = COLORS.subtext0;
pub const SUBTEXT_1: ThemeColor = COLORS.subtext1;
pub const SUBTLE: ThemeColor = COLORS.overlay1;
pub const ON_ACCENT: ThemeColor = COLORS.base;
pub const CURSOR: ThemeColor = COLORS.rosewater;

pub const ACTIVE_BORDER: ThemeColor = COLORS.lavender;
pub const INACTIVE_BORDER: ThemeColor = COLORS.overlay0;

pub const COLOR_0: ThemeColor = COLORS.surface1;
pub const COLOR_1: ThemeColor = COLORS.red;
pub const COLOR_2: ThemeColor = COLORS.green;
pub const COLOR_3: ThemeColor = COLORS.yellow;
pub const COLOR_4: ThemeColor = COLORS.blue;
pub const COLOR_5: ThemeColor = COLORS.pink;
pub const COLOR_6: ThemeColor = COLORS.teal;
pub const COLOR_7: ThemeColor = COLORS.subtext0;

pub const COLOR_8: ThemeColor = BRIGHT_COLORS.surface2;
pub const COLOR_9: ThemeColor = BRIGHT_COLORS.red;
pub const COLOR_10: ThemeColor = BRIGHT_COLORS.green;
pub const COLOR_11: ThemeColor = BRIGHT_COLORS.yellow;
pub const COLOR_12: ThemeColor = BRIGHT_COLORS.blue;
pub const COLOR_13: ThemeColor = BRIGHT_COLORS.pink;
pub const COLOR_14: ThemeColor = BRIGHT_COLORS.teal;
pub const COLOR_15: ThemeColor = BRIGHT_COLORS.subtext1;

pub const COLOR_16: ThemeColor = COLORS.peach;
pub const COLOR_17: ThemeColor = COLORS.rosewater;

pub fn to_ratatui(c: ThemeColor) -> RatatuiColor {
    RatatuiColor::Rgb(c.rgb.r, c.rgb.g, c.rgb.b)
}
