# Norris Jokes API

[![latest version](https://img.shields.io/crates/v/norris-jokes.svg?style=flat-square)](https://crates.io/crates/norris-jokes)
[![build status](https://img.shields.io/github/actions/workflow/status/skewnart/norris-jokes-api/ci.yml?style=flat-square)](https://github.com/Skewnart/norris-jokes-api/actions)
[![dependency status](https://deps.rs/repo/github/skewnart/norris-jokes-api/status.svg)](https://deps.rs/repo/github/skewnart/norris-jokes-api)
[![downloads](https://img.shields.io/crates/d/norris-jokes.svg?style=flat-square)](https://crates.io/crates/norris-jokes)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/norris-jokes)
![license](https://img.shields.io/crates/l/norris-jokes.svg?style=flat-square)

[Chuck Norris jokes API](https://api.chucknorris.io/) sync & async fetcher library for Rust applications

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
