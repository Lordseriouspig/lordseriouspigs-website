// Copyright (C) 2026 Lordseriouspig
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write};
use tokio::sync::broadcast;

pub struct WsWriter {
    pub tx: broadcast::Sender<Vec<u8>>,
    pub buffer: Vec<u8>,
}

impl Write for WsWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let frame = std::mem::take(&mut self.buffer);
            eprintln!("Sent frame: {} bytes", frame.len());
            let _ = self.tx.send(frame);
        }
        Ok(())
    }
}