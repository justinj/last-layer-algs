use generator::{Generator};
use cubestate::{CubeState};
use algorithm::{Algorithm};
use ::std::str::FromStr;

pub struct AlgorithmIterator {
    cubestates: Vec<CubeState>,
    moves: Vec<Generator>,
    indices: Vec<usize>,
    length: i8
}

impl AlgorithmIterator {
    pub fn new() -> Self {
        let first_move = Generator::first();
        let mut iter = AlgorithmIterator {
            moves: vec![*first_move],
            cubestates: vec![first_move.effect],
            indices: vec![0],
            length: 6,
        };

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
            // we have this gross special case because the legal starting moves are a special case
            self.indices[0] += 1;
            if self.indices[0] >= Generator::starting_moves().len() {
                return None;
            } else {
                self.cubestates[0] = Generator::starting_moves()[self.indices[0]].effect.clone();
                self.moves[0] = Generator::starting_moves()[self.indices[0]].clone();
                return Some(self.cubestates[0])
            }
        }
        let preceding_move = self.moves[idx];
        self.indices[idx] += 1;
        if self.indices[idx] >= preceding_move.successors().len() {
            self.indices[idx] = 0;
            if let None = self.inc_idx(idx - 1) {
                return None;
            }
        }

        self.moves[idx] = *self.moves[idx - 1].successors()[self.indices[idx]];

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
    type Item = Algorithm;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_cube = self.cubestates[self.cubestates.len() - 1];

        while self.moves[self.moves.len() - 1].is_u_move() || !current_cube.is_ll() {
            if let Some(cube) = self.increment_to_next_cube() {
                current_cube = cube;
            } else {
                return None;
            }
        }

        let alg = self.current_algorithm();
        self.increment_to_next_cube();

        // TODO: dont unwrap here
        let algorithm = Algorithm::from_str(alg.as_str()).unwrap();
        Some(algorithm.best_rotation())
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    #[test]
    fn test_6_movers() {
        assert_eq!(
            ::algorithm_iterator::AlgorithmIterator::new().map(|a| format!("{}", a)).collect::<Vec<String>>(),
            vec!["F U R U' R' F'", "F R U R' U' F'", "R' U' F' U F R", "R' F' U' F U R"]
        );
    }

    #[bench]
    fn bench_gen_6s(b: &mut Bencher) {
        b.iter(|| {
            for alg in ::algorithm_iterator::AlgorithmIterator::new() {
                ::test::black_box(alg);
            }
        });
    }
}
