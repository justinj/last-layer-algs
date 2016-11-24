#![allow(dead_code)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate clap;

use clap::{Arg, App, SubCommand};

mod generator;
mod cubestate;
mod algorithm;
mod algorithm_iterator;

use algorithm_iterator::{AlgorithmIterator};
use std::io::Write;

fn alg_following(s: &str) -> Result<String, String> {
    let mut it = AlgorithmIterator::from_starting_algorithm(s)?;
    let alg = it.next().unwrap();
    Ok(format!("{}", alg))
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
}
    // for alg in AlgorithmIterator::new() {
    //     println!("{} is an LL alg!", alg);
    // }
