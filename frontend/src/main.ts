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

import {Terminal} from "@xterm/xterm";
import {FitAddon} from "@xterm/addon-fit";
import "@xterm/xterm/css/xterm.css";

// Setup stuff
const app = document.querySelector<HTMLDivElement>("#app");
if (!app) throw new Error("App element not found");

app.style.width = "100vw";
app.style.height = "100vh";
app.style.margin = "0";
app.style.background = "#111";

const termContainer = document.createElement("div");
termContainer.style.width = "100%";
termContainer.style.height = "100%";
app.appendChild(termContainer);

const term = new Terminal({
    cursorBlink: false,
    convertEol: true,
})

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

term.open(termContainer);
fitAddon.fit();

// Try connect
try {
    const response =
        await fetch(
            "http://192.168.1.64:4000/api/session",
            {
                method: "POST",
            }
        );
    if (!response.ok) {
        term.clear()
        term.writeln(`Unable to connect to the server - HTTP Error ${response.status}, ${response.statusText}`);
        throw new Error(`HTTP Error ${response.status} on connect to session server.`)
    }
    const data =
        await response.json();
    const sessionId =
        data.session_id;

    const ws = new WebSocket(`ws://192.168.1.64:4000/ws/${sessionId}`);
    ws.binaryType = "arraybuffer";

    ws.onopen = () => {
        console.log("Connected to TUI backend :yayayayayay:");

        const cols = term.cols;
        const rows = term.rows;

        const msg = JSON.stringify({
            type: "resize",
            cols,
            rows
        });

        console.debug(`sent initial resize event: ${msg}`);
        ws.send(msg);
    };

    ws.onmessage = async (event) => {
        const data = new Uint8Array(event.data);
        term.write(data);
    }

    ws.onclose = () => {
        term.clear()
        console.warn(`Connection Closed`);
        term.writeln(`Connection Closed`);
    }

    ws.onerror = (error) => {
        term.clear()
        console.error(`WebSocket connection Error: ${error}`);
        term.writeln(`Connection Error. Please refresh the page.`);
    }

    window.addEventListener("resize", () => {
        fitAddon.fit();

        const cols = term.cols;
        const rows = term.rows;

        const msg = JSON.stringify({
            type: "resize",
            cols,
            rows
        });

        console.debug(`sent resize event: ${msg}`);
        ws.send(msg);
    });

    term.onKey((e) => {
        let key = e.domEvent.key
        if (!key) return;
        if (key == 'r') {
            fitAddon.fit();
            const cols = term.cols;
            const rows = term.rows;

            const msg = JSON.stringify({
                type: "resize",
                cols,
                rows
            });

            console.log(`manually sent resize event: ${msg}`);
            ws.send(msg);
        }
        const msg = JSON.stringify({
            type: "key",
            key: key
        });
        ws.send(msg);
        console.debug(`sent key event: ${msg}`);
    })
} catch (e) {
    term.clear()
    console.error(`An error occurred: ${e}`);
    term.writeln("An error occurred. Please check the console for more info.");
}