extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate clap;
extern crate image;

use std::path::Path;
use clap::{Arg, App};
use std::fs::File;

use std::str::from_utf8;

// make a comic type to utilize the serde library full in deserializing of json from api
#[derive(Debug, Deserialize)]
struct Comic {
    num: u32,
    title: String,
    img: String,
}


fn main() {
    // Command line setup with clap to imprive interface
    let matches = App::new("xkcd_ascii")
                    .version("0.0.1")
                    .author("Github - @JamesVStone")
                    .about("Fetch XKCD comic and print in ascii")
                    .args(&[
                        Arg::from_usage("[INPUT] The comic to display")
                    ])
                    .get_matches();
    
    let comic_id: String = matches.value_of("INPUT").unwrap().to_string(); 
    // TODO: select random number if arg not present
    
    let latest_comic: Comic = get_comic_info("https://xkcd.com/info.0.json");
    

    // evaluate if the requested comic is within the range of released comics
    if comic_id.parse::<u32>().unwrap() > latest_comic.num {
        panic!("The entered comic number is not avadiable");
    }

    let comic_data_url = format!("https://xkcd.com/{}/info.0.json", comic_id);

    let comic: Comic = get_comic_info(&comic_data_url);

    println!("{}", comic.title);

    let path = get_comic_image(&comic.img); 
    to_ascii(&path, vec![160u32, 80u32]);
    std::fs::remove_file(&path);
}

fn get_comic_info(url: &str) -> Comic {
    let mut res = reqwest::get(url).unwrap();
    let x: Comic = res.json().unwrap();
    x
}

fn get_comic_image(url: &str) -> String {
    let mut req = reqwest::get(url).unwrap();
    let filetype :Vec<&str> = url.split(".").collect();
    let file_name = format!("./temp.{}", &filetype[filetype.len() -1]);
    let mut file = File::create(&file_name).unwrap();
    std::io::copy(&mut req, &mut file);
    file_name
}

/*
    Below code has been made by github user edelsonc.
    You can find it here: https://github.com/edelsonc/asciify
*/

fn intensity_to_ascii(value: &u8) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];

    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}
fn to_ascii(image_name: &String, dims: Vec<u32>) {
    let img = match image::open(Path::new(&image_name)) {
        Ok(p) => p,
        Err(_e) => panic!("Not a valid image path or could no open image"),
    };
    // resize image as an option if its very large...defualts to screen width
    let img = img.resize_exact(dims[0], dims[1], image::FilterType::Nearest);

    // convert to LUMA and change each greyscale pixel into a character
    let imgbuf = img.to_luma();
    let ascii_art = imgbuf.pixels()
        .map( |p| intensity_to_ascii(&p[0]) )
        .fold( String::new(), |s, p| s + p );

    // we have one long string, but we need to chunk it by line
    let subs = ascii_art.as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .collect::< Result <Vec < & str >, _ > > ()
        .unwrap();
    for s in subs {
        println ! ("{}", s);
    }
}