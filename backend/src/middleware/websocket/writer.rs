use std::io::{self, Write};
use tokio::sync::broadcast;

pub struct WsWriter {
    pub tx: broadcast::Sender<Vec<u8>>,
}

impl Write for WsWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // send raw ANSI bytes
        let _ = self.tx.send(buf.to_vec());
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}