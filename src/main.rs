#![allow(dead_code)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

mod generator;
mod cubestate;
mod algorithm;
mod algorithm_iterator;

use algorithm_iterator::{AlgorithmIterator};

fn main() {
    for alg in AlgorithmIterator::new() {
        println!("{} is an LL alg!", alg);
    }
}

