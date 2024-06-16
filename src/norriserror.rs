use std::fmt;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum NorrisError {
    Request(ReqwestError),
    Json(SerdeError),
    Generic(String)
}

impl fmt::Display for NorrisError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        use NorrisError::*; 
 
        match self { 
            Request(err) => err.fmt(f),
            Json(err) => err.fmt(f),
            Generic(err) => write!(f, "{}", err) 
        } 
    } 
}
