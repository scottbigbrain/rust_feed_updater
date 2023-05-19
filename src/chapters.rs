use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Chapter {
    pub title: String,
    pub startTime: String,
}


