pub enum JokeCategory {
    
}

impl JokeCategory {
    pub fn to_request(&self) -> String {
        self.to_string().to_lowercase();
    }
}