# Norris Jokes API

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
