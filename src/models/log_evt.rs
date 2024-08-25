#[derive(Clone, Debug)]
pub struct LogEvent {
    pub timestamp: String,
    pub entity: String,
    pub data: String,
}