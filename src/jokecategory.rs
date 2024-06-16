use std::fmt;

#[derive(Debug)]
pub enum JokeCategory {
    Animal,
    Career,
    Celebrity,
    Dev,
    Explicit,
    Fashion,
    Food,
    History,
    Money,
    Movie,
    Music,
    Political,
    Religion,
    Science,
    Sport,
    Travel
}

impl fmt::Display for JokeCategory { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    } 
}