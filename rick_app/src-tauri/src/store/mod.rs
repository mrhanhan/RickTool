use serde::{Deserialize, Serialize};

pub mod app;


#[derive(Serialize, Deserialize)]
pub struct Id {
    pub id: i32
}