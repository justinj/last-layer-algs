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

// TODO: take this as a cli param?
const LAST_FNAME: &'static str = "last";

fn alg_following(s: &str) -> Result<String, LLAError> {
    // for a in AlgorithmIterator::new() {
    //     println!("{}", a);
    // }
    let mut it = AlgorithmIterator::from_starting_algorithm(s)?;
    let alg = it.next().unwrap();
    Ok(format!("{}", alg))
}

fn prepare_tweet() {
    let path = Path::new(LAST_FNAME);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open last alg file {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => s = String::from(s.trim()),
        Err(why) => panic!("Couldn't read file: {}", why),
    }

    let result = match alg_following(s.as_str()) {
        Err(why) => {
            writeln!(&mut std::io::stderr(), "Error: {}", why).unwrap();
            std::process::exit(1)
        },
        Ok(alg) => alg
    };

    let alg = Algorithm::from_str(result.as_str()).unwrap().inverse();
    ::image_generator::generate_image(alg.cube(), "output_file.png");

    match ::tweet::tweet(format!("{}", result).as_str()) {
        Err(why) => println!("Couldn't tweet: {}", why),
        Ok(()) => {
            match File::create(&path) {
                Err(why) => panic!("Couldn't open file for writing: {}", why),
                Ok(mut file) => {
                    match file.write_all(format!("{}", result).as_bytes()) {
                        Err(why) => panic!("Couldn't write to file: {}", why),
                        Ok(_) => ()
                    }
                }
            };
        }
    };
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
        prepare_tweet();
    }
    
    for alg in AlgorithmIterator::new() {
        println!("{} is an LL alg!", alg);
    }
}
