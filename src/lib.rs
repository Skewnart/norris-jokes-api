mod models;
use crate::models::joke::*;

pub fn add(left: usize, right: usize) {
    // left + right

    let test = Joke::new("bmom6jqftpqgokh8adtolw", "Chuck Norris once rode a nine foot grizzly bear through an automatic car wash, instead of taking a shower.");
    println!("{}", test);
}

pub fn get_random() -> String {
    let client = reqwest::blocking::Client::new(); 

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
