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
use crate::tui::ui::components::{border_paragraph, default_block, default_heading, default_inner};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::Block;

pub fn render(border: Block<'static>, area: Rect, buf: &mut Buffer) {
    // Strings
    let title = r#"     _    _                 _     _   _     _           _ _
    / \  | |__   ___  _   _| |_  | |_| |__ (_)___   ___(_) |_ ___
   / _ \ | '_ \ / _ \| | | | __| | __| '_ \| / __| / __| | __/ _ \
  / ___ \| |_) | (_) | |_| | |_  | |_| | | | \__ \ \__ \ | ||  __/
 /_/   \_\_.__/ \___/ \__,_|\__|  \__|_| |_|_|___/ |___/_|\__\___|
                                                                  "#;
    let p1 = vec![
        Line::from(vec![Span::raw(
            "In the rawest of senses, I've pretty much just written a transport to view a terminal in the web, but it does a bit more then that. When you connected to this website (a vite frontend, as a reminder), your browser made a GET request to `/api/session`. This triggered the backend to create a session for you, and return the session ID back to you. This is all handled in `./src/middleware/websocket/server.rs`. Your browser will then connect to `/api/ws/:id` whilst the server spins up an instance of this TUI linked to your session, with a custom crossterm writer to move things from ratatui to the websocket server. A lot of making this sort of stuff involved replacing things that crossterm would normally take from stdin, like the keystrokes and terminal sizes. Once you connect to the websocket, your client will send a client-ready signal, which signals the tui to start rendering. Ratatui does its magic, and eventually sends a frame to the websocket server, which forwards it back on to you for xtermjs to render! Whenever you send an input, which can be a key press, window resize, or a few others, it'll be sent back through the websocket and back to runtime.rs, which will process it accordingly (ie update the app state to resize the terminal or change the active page or whatever). When you decide to leave this page and close the websocket connection, the server will detect that and destroy everything. Writing this I've just realised that I've forgotten (or more likely couldnt've been bothered) to write something that closes the websocket on an internal error. Oh well.",
        )]),
        Line::from(vec![]),
        Line::from(vec![
            Span::raw(
                "Most of the time I've spent on this has probably been on sessioning. The amount of rust things that I had to work through (if you have ever coded in rust, you'd know what I mean), not to mention the time on figuring out how tokio worked, made it really hard to do, and I'd often find myself refactoring a lot because I did not go into this with a plan. There were also some weird issues with terminal sizing that took ages to figure out. One was that I was chaging the size of the main widget inside of the terminal and not the size of the \"terminal\" itself. I spent ages trying to figure out why the size was capped at a certain spot until I subconsiously resized the terminal it was running in and saw the size change. Another issue was that I was losing frames between the writer and the websocket server, but ",
            ),
            Span::styled("only ", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(
                "when the user's screen size was more than 64 columns long. The good ol' breakpoints helped me figure out where these frames were being dropped and that my broadcast channel was lagging, so I upped the size of it and added some more error handling that I'd forgotten to add before.",
            ),
        ]),
        Line::from(vec![]),
        Line::from(vec![Span::raw(
            "Anywayssssss, that was the story of my last week after I'd locked in to get my hours in time for Horizons Crux. Knowing me this temporary body text will probably stay forever, but if you read through all that, I hope you didnt go insane and I hope you somewhat enjoyed that (is that something people enjoy reading lmao?). If not, I hope you found that somewhat interesting at least. I've never seen this sort of thing done before (other then like in proper terminal emulation but that doesn't count) so I wanted to give it a try. Also if this dosn't fit well on your screen, uh, sorry :pf:.",
        )]),
    ];

    let (border, inner_area) = default_inner(border, area);
    border.render(area, buf);
    let block = default_block();
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(inner_area);
    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(6), Constraint::Fill(1)])
        .split(layout[1]);

    default_heading(title, block.clone()).render(inner[0], buf);
    border_paragraph(p1, block.clone()).render(inner[1], buf);
}
