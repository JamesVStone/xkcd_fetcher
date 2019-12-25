extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Comic {
    num: u32,
    title: String,
    img: String,
}


fn main() {
    let comic_id = env::args().nth(1);

    
}

fn get_comic_info(url: &str) -> Comic {
    let client = reqwest::Client::new();
    let mut res = client.get(url).send().unwrap();
    let x: Comic = res.json().unwrap();
    x
}