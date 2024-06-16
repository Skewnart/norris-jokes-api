mod joke;
mod jokecategory;
mod norriserror;
mod requestor;

use requestor::Requestor;
use joke::{Joke, MultiJokes};
use jokecategory::JokeCategory;
use norriserror::NorrisError;

pub fn get_random() -> Result<Joke, NorrisError> {
    Requestor::new()
        .retrieve_response_sync("/random")
        .parse_it_to::<Joke>()
}

pub fn get_random_with_category(category: JokeCategory) -> Result<Joke, NorrisError> {
    Requestor::new()
        .retrieve_response_sync(format!("/random?category={}", category).as_str())
        .parse_it_to::<Joke>()
}

pub fn get_with_query(query: &str) -> Result<MultiJokes, NorrisError> {
    Requestor::new()
        .retrieve_response_sync(format!("/search?query={}", query).as_str())
        .parse_it_to::<MultiJokes>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random() {
        assert!(get_random().is_ok());
    }

    #[test]
    fn test_random_with_category() {
        assert!(get_random_with_category(JokeCategory::Sport).is_ok());
    }

    #[test]
    fn test_with_query() {
        let mut jokes = get_with_query("forsureitdoesnotexist");
        assert!(jokes.is_ok());

        if let Ok(res) = jokes {
            assert_eq!(res.total, 0);
        }

        jokes = get_with_query("sport");
        assert!(jokes.is_ok());

        if let Ok(res) = jokes {
            assert!(res.total > 0);
        }
    }
}
