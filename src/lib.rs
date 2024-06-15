mod joke;
mod jokecategory;
mod norriserror;
mod request;

use request::RequestClient;
use joke::Joke;
use norriserror::NorrisError;

pub fn get_random() -> Result<Joke, NorrisError> {
    RequestClient::new()
        .retrieve_response_sync("/random")
        .parse_it::<Joke>()
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
