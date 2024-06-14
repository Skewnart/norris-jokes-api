mod joke;
mod jokecategory;
mod norriserror;

use joke::Joke;
use norriserror::NorrisError;
use reqwest::StatusCode;

pub fn get_random() -> Result<Joke, NorrisError> {    
    let response = reqwest::blocking::get("https://api.chucknorris.io/jokes/random").unwrap();

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

    let joke: Result<Joke, serde_json::Error> = serde_json::from_str(json_text.as_str());
    // println!("joke: {:?}", joke);

    match joke {
        Ok(joke) => Ok(joke),
        Err(err) => Err(NorrisError::Json(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_client() {
        
        match get_random() {
            Ok(joke) => println!("{}", joke),
            Err(err) => println!("{}", err)
        }
    }
}
