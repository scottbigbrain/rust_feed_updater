#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::fs;
use serde::{Serialize, Deserialize};

use crate::episode::*;
use crate::chapters::*;
pub mod episode;
pub mod chapters;

fn main() {
    let file_str = fs::read_to_string("./examples/test.yaml").expect("Unable to read file.");
    let episode: Episode = serde_yaml::from_str(&file_str).expect("Unable to parse YAML.");

    // println!("{:?}", episode);
    println!("{:?}", fs::metadata("./examples/test.yaml").unwrap().len());
}
