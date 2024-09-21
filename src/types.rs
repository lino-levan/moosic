use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Song {
    pub name: String,
    pub file: String,
}
