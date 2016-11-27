#![allow(dead_code)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate clap;
extern crate cairo;
extern crate twitter_api as twitter;
extern crate oauth_client as oauth;
extern crate rustc_serialize;

use clap::{Arg, App, SubCommand};

mod generator;
mod cubestate;
mod algorithm;
mod algorithm_iterator;
mod tweet;
mod image_generator;
mod lla_error;

use algorithm::Algorithm;
use ::std::str::FromStr;
use algorithm_iterator::{AlgorithmIterator};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use ::lla_error::LLAError;
use std::error::Error;

// TODO: take this as a cli param?
const LAST_FNAME: &'static str = "last";
const IMAGE_FNAME: &'static str = "output_file.png";

fn alg_following(s: &str) -> Result<String, Box<Error>> {
    // for a in AlgorithmIterator::new() {
    //     println!("{}", a);
    // }
    let mut it = AlgorithmIterator::from_starting_algorithm(s)?;
    let alg = it.next().unwrap();
    Ok(format!("{}", alg))
}

fn get_last_alg() -> Result<String, Box<Error>> {
    let path = Path::new(LAST_FNAME);
    let mut file = File::open(&path)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn write_alg(alg: String) -> Result<(), Box<Error>> {
    let path = Path::new(LAST_FNAME);
    File::create(&path)?.write_all(alg.as_bytes())?;
    Ok(())
}

fn prepare_tweet() -> Result<(), Box<Error>> {
    let s = get_last_alg()?;
    let alg_to_tweet = alg_following(s.as_str())?;
    let inverted_alg = Algorithm::from_str(alg_to_tweet.as_str()).unwrap().inverse();
    ::image_generator::generate_image(inverted_alg.cube(), IMAGE_FNAME);
    ::tweet::tweet(alg_to_tweet.as_str(), IMAGE_FNAME)?;
    write_alg(alg_to_tweet)?;

    Ok(())
}

fn main() {
    let matches = App::new("Last Layer Algs")
        .version("0.1")
        .author("Justin Jaffray")
        .about("Generates last layers")
        .subcommand(SubCommand::with_name("following")
                    .about("Prints out the alg following the given one")
                    .arg(Arg::with_name("alg")
                         .help("the algorithm to follow")
                         .index(1)
                         .required(true)))
        .subcommand(SubCommand::with_name("tweet")
                    .about("Tweet out the next alg"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("following") {
        if let Some(alg) = matches.value_of("alg") {
            match alg_following(alg) {
                Ok(next) => println!("{}", next),
                Err(msg) => {
                    writeln!(&mut std::io::stderr(), "Error: {}", msg).unwrap();
                    std::process::exit(1);
                }
            }
        }
    }

    if let Some(_) = matches.subcommand_matches("tweet") {
        match prepare_tweet() {
            Err(why) => panic!("Error: {}", why),
            Ok(()) => {}
        }
    }
}
