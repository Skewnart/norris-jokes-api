# Norris Jokes API  ([crates.io](https://crates.io/crates/norris-jokes))

[Chuck Norris jokes API](https://api.chucknorris.io/) sync fetcher library for Rust applications

## Usage

```rust
use norris_jokes::jokecategory::JokeCategory;

fn main() {
    let mut result = norris_jokes::get_random();
    println!("{:?}", result);
    
    result = norris_jokes::get_random_with_category(JokeCategory::Sport);
    println!("{:?}", result);
    
    let result_list = norris_jokes::get_with_query("sport");
    println!("{:?}", result_list);
}
```

## What's next ?

As soon as the Norris API come back (currently down), I code the async versions of functions.
If it's up but no async functions in the project, [tell me](https://github.com/Skewnart/norris-jokes-api/issues), maybe I don't know it, thanks.
