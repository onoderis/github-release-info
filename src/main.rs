extern crate reqwest;

use std::env;
use std::error::Error;

use serde::Deserialize;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let user = &args[0];
    let project = &args[1];

    let url = format!("https://api.github.com/repos/{}/{}/releases", user, project);
    let response: Vec<Release> = reqwest::get(url)?
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
