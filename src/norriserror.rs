use std::fmt;

use reqwest::Error as ReqwestError;

pub enum NorrisError {
    RequestError(ReqwestError)
}

impl fmt::Display for NorrisError { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        use NorrisError::*; 
 
        match self { 
            RequestError(err) => err.fmt(f),
        } 
    } 
} 

/*impl From<ReqwestError> for NorrisError {
    fn from(err: ReqwestError) -> Self { 
        err.
    } 
}
*/