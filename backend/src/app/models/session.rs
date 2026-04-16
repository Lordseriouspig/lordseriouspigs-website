use tokio::sync::mpsc::UnboundedSender;

pub struct Session {
    pub id: String,
    pub tx: UnboundedSender<Vec<u8>>,
}