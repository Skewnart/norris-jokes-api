mod joke;
mod jokecategory;
mod norriserror;

use joke::Joke;
use jokecategory::JokeCategory;
use norriserror::NorrisError;
use reqwest::{blocking::Response, Error as ReqwestError};

pub fn get_random() -> Result<Joke, NorrisError> {
    // let test = Joke::new("bmom6jqftpqgokh8adtolw", "Chuck Norris once rode a nine foot grizzly bear through an automatic car wash, instead of taking a shower.");
    
    let response: Result<Response, ReqwestError> = reqwest::blocking::get("https://api.chucknorris.io/jokes/random");
    let content = match response {
        Ok(response) => response.text(),
        Err(err) => panic!("{}", NorrisError::RequestError(err))
    };
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
