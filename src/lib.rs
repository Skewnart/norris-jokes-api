mod joke;
mod jokecategory;
mod norriserror;
mod request;

use request::RequestClient;
use joke::Joke;
use jokecategory::JokeCategory;
use norriserror::NorrisError;

pub fn get_random() -> Result<Joke, NorrisError> {
    RequestClient::new()
        .retrieve_response_sync("/random")
        .parse_it::<Joke>()
}

pub fn get_random_category(category: JokeCategory) -> Result<Joke, NorrisError> {
    RequestClient::new()
        .retrieve_response_sync(format!("/random?category={}", category.to_request()).as_str())
        .parse_it::<Joke>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        match get_random() {
            Ok(joke) => println!("{}", joke),
            Err(err) => println!("{}", err)
        }
    }

    #[test]
    fn test_random_category() {
        match get_random_category(JokeCategory::Food) {
            Ok(joke) => println!("{}", joke),
            Err(err) => println!("{}", err)
        }
    }
}
