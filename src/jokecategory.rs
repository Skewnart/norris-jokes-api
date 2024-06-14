use std::fmt;

#[derive(Debug)]
pub enum JokeCategory {
    
}

impl fmt::Display for JokeCategory { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{:?}", self) 
    } 
}

impl JokeCategory {
    pub fn to_request(&self) -> String {
        self.to_string().to_lowercase()
    }
}