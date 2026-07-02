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

use std::vec;

use crate::tui::ui::components::{default_heading, default_inner, default_paragraph};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};

/*
Page design guide (mainly for me, you can ignore this. I'll update this as I need)
Page titles should be made with the Standard font on https://patorjk.com/software/taag/#p=display&f=Standard&t=Title+font&x=none&v=4&h=4&w=80&we=false.
Pages should use components in ./ui/components.rs and widgets should be placed in ./ui/widgets.rs

The structure of a view file should be:
- Strings
- Layout
- Content
 */

pub fn render(border: Block<'static>, area: Rect, buf: &mut Buffer) {
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
                "Yup, that's right, this website is built entirely in rust! (well apart from a small vite server to serve the frontend). It's nowhere near done, but most of the stuff I still have to do is on the tui, not the actual server. This site might end up looking a lotttttt different soon as I inevitably decide I'm not happy with something and redesign everything.",
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

    Paragraph::new(title_1)
        .block(block.clone())
        .render(inner[0], buf);
    default_heading(title_2, block.clone()).render(inner[1], buf);
    default_paragraph(p1, block.clone()).render(inner[2], buf);
    // TODO: Hackatime stats w/ loading anim
    // TODO: Right column w/ links, stats, github stuff, slack api stuff, time, etc
}
