use std::fmt;
use std::error;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum NorrisError {
    RequestError(ReqwestError),
    JsonError(SerdeError)
}

impl fmt::Display for NorrisError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        use NorrisError::*; 
 
        match self { 
            RequestError(err) => err.fmt(f),
            JsonError(err) => err.fmt(f),
        } 
    } 
}

impl error::Error for NorrisError {
    fn description(&self) -> &str {
        use NorrisError::*;

        match self { 
            RequestError(_) => "Erreur de requête",
            JsonError(_) => "Erreur de parsing JSON",
        } 
    }
}
