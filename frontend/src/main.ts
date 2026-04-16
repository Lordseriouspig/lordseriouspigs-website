import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import "xterm/css/xterm.css";

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
  cursorBlink: true,
  convertEol: true,
})

const fitAddon = new FitAddon();
term.loadAddon(fitAddon);

term.open(termContainer);
fitAddon.fit();

window.addEventListener("resize", () => {
  fitAddon.fit();
});

const ws = new WebSocket("ws://localhost:4000/ws");
ws.binaryType = "arraybuffer";

ws.onopen = () => {
  console.log("Connected to TUI backend :yayayayayay:");
};

ws.onmessage = (event) => {
  const data = new Uint8Array(event.data);
  term.write(data);
}

ws.onclose = () => {
  term.writeln("Connection closed");
  console.log("Connection closed");
}

ws.onerror = (error) => {
  term.writeln("Connection error");
  console.error("WebSocket error:", error);
}