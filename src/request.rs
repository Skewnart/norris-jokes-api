
use reqwest::{blocking::Response, StatusCode};
use serde::Deserialize;

use crate::norriserror::NorrisError;

pub struct RequestClient {
    url: String,
    response: Option<Response>
}

impl RequestClient {
    pub fn new() -> Self {
        Self {
            url: "https://api.chucknorris.io/jokes".to_string(),
            response: None
        }
    }
    
    pub fn retrieve_response_sync(mut self, path: &str) -> Self {
        self.response = Some(reqwest::blocking::get(format!("{}{}", self.url, path)).unwrap());
        self
    }

    pub fn parse_it<T>(self) -> Result<T, NorrisError> 
        where T: for<'a> Deserialize<'a>{
        let response = match self.response {
            Some(x) => x,
            None => return Err(NorrisError::Generic("Reponse not loaded yet.".to_string()))
        };

        if response.status() != StatusCode::OK {
            return Err(NorrisError::Generic(format!("{} : {}", response.url().path(), response.status())));
        }
    
        let content: Result<String, reqwest::Error> = response.text();
        // println!("content: {:?}", content);
    
        let json_text: String = match content {
            Ok(text) => text,
            Err(err) => return Err(NorrisError::Request(err))
        };
        // println!("json_text: {:?}", json_text);
    
        let joke: Result<T, serde_json::Error> = serde_json::from_str(json_text.as_str());
        // println!("joke: {:?}", joke);
    
        match joke {
            Ok(joke) => Ok(joke),
            Err(err) => Err(NorrisError::Json(err))
        }
    }
}
