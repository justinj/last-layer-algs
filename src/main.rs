#![allow(dead_code)]
mod generator;
mod cubestate;

use generator::{Generator};
use cubestate::{CubeState};

struct AlgorithmIterator {
    // a bunch of these are unnecessary and this is just generally really messy rn
    cubestates: Vec<CubeState>,
    moves: Vec<Generator>,
    indices: Vec<usize>,
    successors: Vec<Vec<&'static Generator>>,
    length: i8
}

impl AlgorithmIterator {
    fn new() -> Self {
        let moves = vec![Generator::first().clone()];
        let mut iter = AlgorithmIterator {
            moves: moves,
            cubestates: vec![],
            successors: vec![],
            indices: vec![0],
            length: 8,
        };
        iter.cubestates = vec![iter.moves[0].effect];
        iter.successors = vec![iter.moves[0].successors()];

        while iter.moves.len() < iter.length as usize {
            let last = iter.moves[iter.moves.len() - 1];
            iter.push_move(last.successors()[0].clone());
        }
        iter
    }

    fn push_move(&mut self, g: Generator) {
        self.moves.push(g);
        self.indices.push(0);
        self.successors.push(g.successors());
        let last = self.cubestates[self.cubestates.len() - 1];
        self.cubestates.push(last.apply(&g.effect));
    }

    fn inc_idx(&mut self, idx: usize) -> bool {
        if idx == 0 {
            return false;
        }
        self.indices[idx - 1] += 1;
        if self.indices[idx - 1] >= self.successors[idx - 1].len() {
            self.indices[idx - 1] = 0;
            if !self.inc_idx(idx - 1) {
                return false;
            }
        }
        self.moves[idx] = self.successors[idx - 1][self.indices[idx - 1]].clone();
        self.successors[idx] = self.moves[idx].successors();
        self.cubestates[idx] = self.cubestates[idx - 1].apply(&self.moves[idx].effect);
        true
    }
}

impl Iterator for AlgorithmIterator {
    type Item = (::cubestate::CubeState, String);

    fn next(&mut self) -> Option<(CubeState, String)> {
        let last = self.cubestates[self.cubestates.len() - 1];
        let idx = self.moves.len() - 1;
        let move_names: Vec<String> = self.moves.iter().map(|m| m.name()).collect();

        if self.inc_idx(idx) {
            Some((last.clone(), move_names.join(" ")))
        } else {
            None
        }
    }
}

fn make_iter() -> AlgorithmIterator {
    AlgorithmIterator::new()
}


fn main() {
    for (state, alg) in make_iter() {
        if state.is_ll() {
            println!("{} is an LL alg!", alg);
        }
    }
}
