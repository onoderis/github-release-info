extern crate clap;
extern crate core;
extern crate reqwest;
extern crate serde;

use core::fmt;
use std::{env, error};
use std::error::Error;
use std::fmt::Display;
use std::ops::Deref;
use std::string::ParseError;

use clap::{App, Arg, ArgMatches, SubCommand};
use serde::Deserialize;
use serde::export::Formatter;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Github release info")
        .version("0.1.1") //todo version from cargo
        .arg(Arg::with_name("user")
            .short("u")
            .long("user")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("repo")
            .short("r")
            .long("repo")
            .takes_value(true)
            .required(true))
        .get_matches();

    let user = matches.value_of("user").ok_or(NoCliArgumentError)?;
    let repository = matches.value_of("repo").ok_or(NoCliArgumentError)?;

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
