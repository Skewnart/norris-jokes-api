mod joke;
mod jokecategory;

pub fn get_random() -> Result<Joke, NorrisError> {
    // let test = Joke::new("bmom6jqftpqgokh8adtolw", "Chuck Norris once rode a nine foot grizzly bear through an automatic car wash, instead of taking a shower.");
    let client = reqwest::blocking::Client::new();
    let url = "https://api.chucknorris.io/jokes/random";

    let response: Result<Response, ReqwestError> = client.get(url).send(); 

    "".to_string()
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
