# Norris Jokes API

[Chuck Norris jokes API](https://api.chucknorris.io/) sync fetcher library for Rust applications

## Usage

### Usage for synchronous calls

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

### Usage for asynchronous calls

```rust
use norris_jokes::jokecategory::JokeCategory;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut joke = norris_jokes::get_random_async().await;
    println!("{:?}", joke);
    
    joke = norris_jokes::get_random_with_category_async(JokeCategory::Sport).await;
    println!("{:?}", joke);
    
    let result_list = norris_jokes::get_with_query_async("sport").await;
    println!("{:?}", result_list);

    Ok(())
}
```
