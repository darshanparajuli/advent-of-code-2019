use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::{thread, time};

pub mod intcode_computer;

pub fn read_input() -> Result<Vec<String>, io::Error> {
    let arg = env::args()
        .skip(1)
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
    BufReader::new(File::open(arg)?).lines().collect()
}

pub fn delay(millis: u64) {
    let millis = time::Duration::from_millis(millis);
    thread::sleep(millis);
}
