use catppuccin::{Color, FlavorColors};

use crate::tui::models;

const fn modify_color(color: Color) -> Color {
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

pub const BACKGROUND: Color = COLORS.base;
pub const CURSER: Color = COLORS.rosewater;
pub const CURSER_TEXT: Color = COLORS.crust;
pub const ACTIVE_BORDER: Color = COLORS.lavender;
pub const INACTIVE_BORDER: Color = COLORS.overlay0;
pub const BELL_BORDER: Color = COLORS.yellow;

pub const COLOR_0: Color = COLORS.surface1;
pub const COLOR_1: Color = COLORS.red;
pub const COLOR_2: Color = COLORS.green;
pub const COLOR_3: Color = COLORS.yellow;
pub const COLOR_4: Color = COLORS.blue;
pub const COLOR_5: Color = COLORS.pink;
pub const COLOR_6: Color = COLORS.teal;
pub const COLOR_7: Color = COLORS.subtext0;

pub const COLOR_8: Color = BRIGHT_COLORS.surface2;
pub const COLOR_9: Color = BRIGHT_COLORS.surface1;
pub const COLOR_10: Color = BRIGHT_COLORS.red;
pub const COLOR_11: Color = BRIGHT_COLORS.green;
pub const COLOR_12: Color = BRIGHT_COLORS.yellow;
pub const COLOR_13: Color = BRIGHT_COLORS.blue;
pub const COLOR_14: Color = BRIGHT_COLORS.pink;
pub const COLOR_15: Color = BRIGHT_COLORS.teal;
pub const COLOR_16: Color = BRIGHT_COLORS.subtext0;
