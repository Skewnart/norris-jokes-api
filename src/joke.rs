use std::fmt; 
use serde::{Deserialize, Serialize}; 
 
#[derive(Clone, Serialize, Deserialize, Debug)] 
pub struct Joke {
    pub id: String,
    pub value: String
}

impl Joke {
    pub fn new(id: &str, value: &str) -> Self {
        Self {
            id: id.to_string(),
            value: value.to_string()
        }
    }
}

impl fmt::Display for Joke { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{:?}", self) 
    } 
}
