use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub children: Vec<Canvas>,
}

#[derive(Debug, Deserialize)]
pub struct Canvas {
    pub id: String,
    pub name: String,
    pub children: Vec<Frame>,
}

#[derive(Debug, Deserialize)]
pub struct Frame {
    pub id: String,
    pub name: String,
    pub children: Option<Vec<Frame>>,
}
