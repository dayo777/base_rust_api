// Add Models here!
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Model1 {
    pub field1: String,
    pub field2: String,
    pub field3: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Model2 {
    pub field1: String,
    pub field2: String,
    pub field3: String,
}