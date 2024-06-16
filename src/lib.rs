mod joke;
mod jokecategory;
mod norriserror;
mod requestor;

use requestor::Requestor;
use joke::Joke;
use jokecategory::JokeCategory;
use norriserror::NorrisError;

pub fn get_random() -> Result<Joke, NorrisError> {
    Requestor::new()
        .retrieve_response_sync("/random")
        .parse_it_to::<Joke>()
}

pub fn get_random_category(category: JokeCategory) -> Result<Joke, NorrisError> {
    Requestor::new()
        .retrieve_response_sync(format!("/random?category={}", category).as_str())
        .parse_it_to::<Joke>()
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
