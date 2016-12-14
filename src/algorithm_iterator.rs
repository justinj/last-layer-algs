#![allow(dead_code, unused_imports)]
use generator::{Generator, Face, Modifier};
use cubestate::CubeState;
use algorithm::Algorithm;
use ::std::str::FromStr;
use ::std::error::Error;
use f2l_cubestate::F2LCubeState;

#[derive(Debug)]
pub struct AlgorithmIterator {
    cubestates: Vec<F2LCubeState>,
    moves: Vec<Generator>,
    indices: Vec<usize>,
    length: i8
}

impl AlgorithmIterator {
    pub fn new() -> Self {
        let mut iter = AlgorithmIterator {
            moves: vec![],
            cubestates: vec![],
            indices: vec![],
            length: 0,
        };

        iter.initialize_with_length(6);
        iter
    }

    fn initialize_with_length(&mut self, len: i8) {
        let first_move = Generator::first();
        self.moves = vec![*first_move];
        self.cubestates = vec![F2LCubeState::new().apply(*first_move)];
        self.indices = vec![0];
        self.length = len;
        while self.moves.len() < self.length as usize {
            let last = self.moves[self.moves.len() - 1];
            self.push_move(last.successors()[0].clone());
        }
    }

    pub fn from_starting_algorithm(s: &str) -> Result<Self, Box<Error>> {
        let alg = Algorithm::from_str(s)?.canonical_rotation();
        let moves = alg.moves.clone();

        if moves.len() == 0 {
            return Ok(Self::new());
        }

        let cubestates = alg.cubestates_stack();
        let mut indices = vec![];

        indices.push(match alg.first_non_ud_move() {
            Some(m) => match m.components() {
                (Face::R, Modifier::Normal) => 0,
                (Face::R, Modifier::Twice)  => 1,
                (Face::R, Modifier::Prime)  => 2,
                _ => panic!("Shouldn't happen because we canonicalized the algorithm above"),
            },
            None => {
                // there was only U's and D's, so the alg can only be
                // "U*", "D*", or "U* D*"
                return Ok(Self::new());
            }
        });

        for &m in moves.iter().skip(1) {
            let cur_idx = indices.len();
            let successors = moves[cur_idx - 1].successors();
            for (i, &&gen) in successors.iter().enumerate() {
                if gen == m {
                    indices.push(i);
                    break;
                }
            }
        }

        Ok(AlgorithmIterator {
            moves: moves,
            cubestates: cubestates,
            indices: indices,
            length: alg.length() as i8,
        })
    }

    fn push_move(&mut self, g: Generator) {
        self.moves.push(g);
        self.indices.push(0);
        let last = self.cubestates[self.cubestates.len() - 1];
        self.cubestates.push(last.apply(g));
    }

    // FIXME: this function needs a lot of work
    fn inc_idx(&mut self, i: usize) -> bool {
        let mut idx = i;
        loop {
            if idx == 0 {
                // we have this gross special case because the legal starting moves are a special case
                self.indices[0] += 1;
                if self.indices[0] >= Generator::starting_moves().len() {
                    return false;
                } else {
                    self.cubestates[0] = F2LCubeState::new().apply(*Generator::starting_moves()[self.indices[0]]);
                    self.moves[0] = Generator::starting_moves()[self.indices[0]].clone();
                    idx += 1;
                    break;
                }
            } else {
                let preceding_move = self.moves[idx - 1];

                self.indices[idx] += 1;

                if self.indices[idx] >= preceding_move.successors().len() {
                    self.indices[idx] = 0;
                    idx -= 1;
                } else {
                    break;
                }
            }
        }

        for idx in idx..(self.length as usize) {
            self.moves[idx] = *self.moves[idx - 1].successors()[self.indices[idx]];

            {
                let (ref left, ref mut right) = self.cubestates.split_at_mut(idx);
                left[idx - 1].apply_into(self.moves[idx], &mut right[0]);
            }

            let distance_to_bottom: u16 = self.length as u16 - 1 - idx as u16;
            if self.cubestates[idx].prunable(distance_to_bottom) {
                return self.inc_idx(idx);
            }
        }

        true
    }

    fn increment_to_next_cube(&mut self) -> F2LCubeState {
        let last_move_index = self.length as usize - 1;
        match self.inc_idx(last_move_index) {
            true => self.cubestates[self.length as usize - 1],
            false => {
                let new_length = self.length + 1;
                self.initialize_with_length(new_length);
                self.increment_to_next_cube()
            },
        }
    }

    fn current_algorithm(&self) -> String {
        self.moves.iter().map(|m| format!("{}", m)).collect::<Vec<String>>().join(" ")
    }

    fn current_cube(&self) -> F2LCubeState {
        self.cubestates[self.cubestates.len() - 1]
    }

    fn ending_in_u_move(&self) -> bool {
        self.moves[self.moves.len() - 1].is_u_move()
            || (self.moves[self.moves.len() - 2].is_u_move()
              && self.moves[self.moves.len() - 1].is_d_move())
    }
}

impl Iterator for AlgorithmIterator {
    type Item = Algorithm;

    fn next(&mut self) -> Option<Self::Item> {
        self.increment_to_next_cube();
        let mut current_cube = self.current_cube();

        while self.ending_in_u_move() || !current_cube.is_ll() {
            current_cube = self.increment_to_next_cube();
        }

        let alg = self.current_algorithm();

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
            ::algorithm_iterator::AlgorithmIterator::new().take(4).map(|a| format!("{}", a)).collect::<Vec<String>>(),
            vec!["F U R U' R' F'", "F R U R' U' F'", "R' U' F' U F R", "R' F' U' F U R"]
        );
    }

    #[test]
    fn test_skips_identities() {
        let alg = ::algorithm_iterator::AlgorithmIterator::
            from_starting_algorithm("L F2 R' F' R F' L'").unwrap()
            .next().unwrap();
        assert_eq!(
            format!("{}", alg),
            "R' U L U' R U L'"
        );
    }

    #[test]
    fn test_from_algorithm_starts_from_certain_position() {
        let first = ::algorithm_iterator::AlgorithmIterator::
            from_starting_algorithm("F R U R' U' F'") .unwrap()
            .next().unwrap();
        assert_eq!(
            format!("{}", first),
            "R' U' F' U F R"
        );
    }

    #[test]
    fn test_increments_length() {
        let first = ::algorithm_iterator::AlgorithmIterator::
            from_starting_algorithm("R' F' U' F U R").unwrap()
            .next().unwrap();
        assert_eq!(
            format!("{}", first),
            "R U R' U R U2 R'"
        );
    }

    //#[bench]
    fn bench_gen_6s(b: &mut Bencher) {
        b.iter(|| {
            for alg in ::algorithm_iterator::AlgorithmIterator::new().take(4) {
                ::test::black_box(alg);
            }
        });
    }
}
