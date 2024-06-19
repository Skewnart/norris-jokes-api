#![doc = include_str!("../README.md")]

pub mod joke;
pub mod jokecategory;
pub mod norriserror;
mod requestor;
mod requestor_async;

use joke::{Joke, MultiJokes};
use jokecategory::JokeCategory;
use norriserror::NorrisError;

pub fn get_random() -> Result<Joke, NorrisError> {
    requestor::Requestor::new()
        .retrieve_response("/random")
        .parse_it_to::<Joke>()
}

pub fn get_random_with_category(category: JokeCategory) -> Result<Joke, NorrisError> {
    requestor::Requestor::new()
        .retrieve_response(format!("/random?category={}", category).as_str())
        .parse_it_to::<Joke>()
}

pub fn get_with_query(query: &str) -> Result<MultiJokes, NorrisError> {
    requestor::Requestor::new()
        .retrieve_response(format!("/search?query={}", query).as_str())
        .parse_it_to::<MultiJokes>()
}

pub async fn get_random_async() -> Result<Joke, NorrisError> {
    requestor_async::Requestor::new()
        .retrieve_response("/random")
        .await
        .parse_it_to::<Joke>()
        .await
}

#[cfg(test)]
mod tests {
    // use std::result;

    use super::*;

    #[test]
    fn test_random() {
        let result_joke = get_random();

        // match result_joke {
        //     Ok(joke) => assert!(!joke.value.is_empty()),
        //     Err(err) => panic!("{:?}", err)
        // }

        if let Ok(joke) = result_joke {
            assert!(!joke.value.is_empty())
        }
    }

    #[test]
    fn test_random_with_category() {
        
        let result_joke = get_random_with_category(JokeCategory::Sport);

        // match result_joke {
        //     Ok(joke) => assert!(!joke.value.is_empty()),
        //     Err(err) => panic!("{:?}", err)
        // }

        if let Ok(joke) = result_joke {
            assert!(!joke.value.is_empty())
        }
    }

    #[test]
    fn test_with_query() {
        let mut result_jokes = get_with_query("forsureitdoesnotexist");
        // match result_jokes {
        //     Ok(jokes) => assert!(jokes.total == 0),
        //     Err(err) => panic!("{:?}", err)
        // }

        if let Ok(jokes) = result_jokes {
            assert!(jokes.total == 0);
        }

        result_jokes = get_with_query("sport");
        // match result_jokes {
        //     Ok(jokes) => assert!(jokes.total > 0),
        //     Err(err) => panic!("{:?}", err)
        // }

        if let Ok(multi) = result_jokes {
            assert!(multi.total > 0);
        }
    }
}
