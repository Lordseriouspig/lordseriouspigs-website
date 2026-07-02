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
use crate::tui::themes::catppuccin::*;
use crate::tui::ui::components::{default_heading, default_inner, default_paragraph};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};
use std::vec;
use tui_piechart::{PieChart, PieSlice};
/*
Page design guide (mainly for me, you can ignore this. I'll update this as I need)
Page titles should be made with the Standard font on https://patorjk.com/software/taag/#p=display&f=Standard&t=Title+font&x=none&v=4&h=4&w=80&we=false.
Pages should use components in ./ui/components.rs.

The structure of a view file should be:
- Strings
- Layout
- Content
 */

pub fn render(border: Block<'static>, area: Rect, buf: &mut Buffer, api_state: &SharedApiState) {
    // Text strings or whatever
    let title_1 = r#" _    _      _ _       _    _____ _
| |  | |    | | |     | |  |_   _( )
| |__| | ___| | | ___ | |    | | |/ _ __ ___
|  __  |/ _ \ | |/ _ \| |    | |   | '_ ` _ \
| |  | |  __/ | | (_) |_|   _| |_  | | | | | |
|_|  |_|\___|_|_|\___/(_)  |_____| |_| |_| |_|"#;
    let title_2 = r#"██╗      █████╗  ██████╗██╗  ██╗██╗      █████╗ ███╗   ██╗
██║     ██╔══██╗██╔════╝██║  ██║██║     ██╔══██╗████╗  ██║
██║     ███████║██║     ███████║██║     ███████║██╔██╗ ██║
██║     ██╔══██║██║     ██╔══██║██║     ██╔══██║██║╚██╗██║
███████╗██║  ██║╚██████╗██║  ██║███████╗██║  ██║██║ ╚████║
╚══════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝
                                                          "#;
    let p1 = vec![
        Line::from(vec![
            Span::raw(
                "Hey! I'm Lachlan, and welcome to my website! I know it might not look like much on the surface, but this website is ",
            ),
            Span::styled(
                "actually running a TUI on the backend! ",
                Style::default().add_modifier(Modifier::ITALIC),
            ),
            Span::raw(
                "Yup, that's right, this website is built entirely in rust! (well apart from a small vite server to serve the frontend). It's nowhere near done, but most of the stuff I still have to do is on the tui, not the actual server. This site might end up looking a lotttttt different soon as I inevitably decide I'm not happy with something and redesign everything. Obviously I'm planning on adding a bit more to this page, I just wanted to get something shipped quickly, I'll probably add some more hackatime stats and neaten some things out, and maybe add some stuff off to the right.",
            ),
        ]),
        Line::from(vec![]),
        Line::from(vec![Span::raw(
            "If you want to learn more about how this website works, you should take a look at the \"About This Site\" page! Be warned it is very rambly. (You can navigate there with either your ◄ ► keys or 'h' and 'l'!",
        )]),
    ];

    let (border, inner_area) = default_inner(border, area);
    border.render(area, buf);
    let block = Block::new().padding(ratatui::widgets::Padding::horizontal(1));
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(inner_area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Fill(1),
            Constraint::Fill(2),
        ])
        .split(layout[0]);
    let hackatime_stats = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner[3]);

    Paragraph::new(title_1)
        .block(block.clone())
        .render(inner[0], buf);
    default_heading(title_2, block.clone()).render(inner[1], buf);
    default_paragraph(p1, block.clone()).render(inner[2], buf);

    // Hackatime Widget. I would have really like to do this in a separate file, but weird sync stuff yk
    let colors = [
        to_ratatui(COLOR_0),
        to_ratatui(COLOR_1),
        to_ratatui(COLOR_2),
        to_ratatui(COLOR_3),
        to_ratatui(COLOR_4),
        to_ratatui(COLOR_5),
        to_ratatui(COLOR_6),
        to_ratatui(COLOR_7),
        to_ratatui(COLOR_8),
        to_ratatui(COLOR_9),
        to_ratatui(COLOR_10),
        to_ratatui(COLOR_11),
        to_ratatui(COLOR_12),
        to_ratatui(COLOR_13),
        to_ratatui(COLOR_14),
        to_ratatui(COLOR_15),
        to_ratatui(COLOR_16),
        to_ratatui(COLOR_17),
    ];

    if let Ok(guard) = api_state.try_read() {
        if let Some(stats) = &guard.stats {
            if let Some(languages) = &stats.data.languages {
                let slices: Vec<PieSlice> = languages
                    .iter()
                    .enumerate()
                    .map(|(i, lang)| {
                        let color = colors[i % colors.len()];
                        PieSlice::new(&lang.name, lang.total_seconds as f64, color)
                    })
                    .collect();
                PieChart::new(slices)
                    .block(Block::bordered().title("Hackatime Language Stats"))
                    .high_resolution(true)
                    .render(hackatime_stats[0], buf);
            }
        }
    };

    // TODO: Neaten up hackatime and cols and stuff
    // TODO: Right column w/ links, stats, github stuff, slack api stuff, time, etc
}
