use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub genre: String,
}
