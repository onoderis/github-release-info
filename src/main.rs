extern crate clap;
extern crate core;
extern crate reqwest;
extern crate serde;

use core::fmt;
use std::error;
use std::error::Error;

use clap::{App, Arg};
use serde::Deserialize;

const USER_NAME_PARAM: &'static str = "user";
const REPO_NAME_PARAM: &'static str = "repo";

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Github release info")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name(USER_NAME_PARAM)
            .short("u")
            .long(USER_NAME_PARAM)
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name(REPO_NAME_PARAM)
            .short("r")
            .long(REPO_NAME_PARAM)
            .takes_value(true)
            .required(true))
        .get_matches();

    let user = matches.value_of(USER_NAME_PARAM).ok_or(NoCliArgumentError)?;
    let repository = matches.value_of(REPO_NAME_PARAM).ok_or(NoCliArgumentError)?;

    let url = format!("https://api.github.com/repos/{}/{}/releases", user, repository);
    let response: Vec<Release> = reqwest::get(&url)?
        .json()?;

    let asset = &response[0].assets[0];
    println!("{} was downloaded {} times", asset.name, asset.download_count);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>
}

#[derive(Deserialize, Debug)]
struct Asset {
    name: String,
    download_count: u32
}

#[derive(Debug, Clone)]
struct NoCliArgumentError;

impl fmt::Display for NoCliArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No required cli argument")
    }
}

impl error::Error for NoCliArgumentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
