use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub children: Vec<Canvas>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Canvas {
    pub id: String,
    pub name: String,
    pub children: Vec<Frame>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Frame {
    pub id: String,
    pub name: String,
    pub children: Option<Vec<Frame>>,
}
