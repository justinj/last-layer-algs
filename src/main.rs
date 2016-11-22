#![allow(dead_code)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;
extern crate test;

mod generator;
mod cubestate;

use generator::{Generator};
use cubestate::{CubeState};

struct AlgorithmIterator {
    // a bunch of these are unnecessary and this is just generally really messy rn
    cubestates: Vec<CubeState>,
    moves: Vec<Generator>,
    indices: Vec<usize>,
    length: i8
}

impl AlgorithmIterator {
    fn new() -> Self {
        let moves = Generator::first();
        let mut iter = AlgorithmIterator {
            moves: vec![*moves],
            cubestates: vec![],
            indices: vec![],
            length: 6,
        };
        iter.cubestates = vec![iter.moves[0].effect];

        while iter.moves.len() < iter.length as usize {
            let last = iter.moves[iter.moves.len() - 1];
            iter.push_move(last.successors()[0].clone());
        }
        iter
    }

    fn push_move(&mut self, g: Generator) {
        self.moves.push(g);
        self.indices.push(0);
        let last = self.cubestates[self.cubestates.len() - 1];
        self.cubestates.push(last.apply(&g.effect));
    }

    fn inc_idx(&mut self, idx: usize) -> Option<CubeState> {
        if idx == 0 {
            return None;
        }
        let preceding_move = self.moves[idx - 1];
        self.indices[idx - 1] += 1;
        if self.indices[idx - 1] >= preceding_move.successors().len() {
            self.indices[idx - 1] = 0;
            if let None = self.inc_idx(idx - 1) {
                return None;
            }
        }

        self.moves[idx] = *self.moves[idx - 1].successors()[self.indices[idx - 1]];

        {
            let (ref left, ref mut right) = self.cubestates.split_at_mut(idx);
            left[idx - 1].apply_into(&self.moves[idx].effect, &mut right[0]);
        }
        Some(self.cubestates[self.cubestates.len() - 1])
    }

    fn increment_to_next_cube(&mut self) -> Option<CubeState> {
        let last_move_index = self.length as usize - 1;
        self.inc_idx(last_move_index)
    }

    fn current_algorithm(&self) -> String {
        self.moves.iter().map(|m| m.name()).collect::<Vec<String>>().join(" ")
    }
}

impl Iterator for AlgorithmIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_cube = self.cubestates[self.cubestates.len() - 1];

        while !current_cube.is_ll() {
            if let Some(cube) = self.increment_to_next_cube() {
                current_cube = cube;
            } else {
                return None;
            }
        }

        let alg = self.current_algorithm();
        self.increment_to_next_cube();
        Some(alg)
    }
}

fn main() {
    for alg in AlgorithmIterator::new() {
        println!("{} is an LL alg!", alg);
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn test_6_movers() {
        assert_eq!(
            ::AlgorithmIterator::new().collect::<Vec<String>>(),
            vec!["R U B U' B' R'", "R B U B' U' R'"]
        );
    }

    #[bench]
    fn bench_gen_6s(b: &mut Bencher) {
        b.iter(|| {
            for alg in ::AlgorithmIterator::new() {
                ::test::black_box(alg);
            }
        });
    }
}
