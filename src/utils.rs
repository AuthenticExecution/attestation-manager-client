use serde::Deserialize;
use std::fs::File;
use anyhow::Result;

use crate::error::Error;

pub fn read_from_file<T: for<'de> Deserialize<'de>>(filename : &str) -> Result<T> {
    read_json(filename).or(read_yaml(filename))
}

fn read_json<T: for<'de> Deserialize<'de>>(filename : &str) -> Result<T> {
    let mut file = match File::open(filename) {
        Ok(f)   => f,
        Err(_)  => return Err(Error::OpenFileError(filename.to_string()).into())
    };


    serde_json::from_reader(&mut file).or_else(|x| Err(x.into()))
}

fn read_yaml<T: for<'de> Deserialize<'de>>(filename : &str) -> Result<T> {
    let mut file = match File::open(filename) {
        Ok(f)   => f,
        Err(_)  => return Err(Error::OpenFileError(filename.to_string()).into())
    };


    serde_yaml::from_reader(&mut file).or_else(|x| Err(x.into()))
}
