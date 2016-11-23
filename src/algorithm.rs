use ::generator::Generator as Generator;
use ::cubestate::CubeState as CubeState;
use ::std::str::FromStr;
use ::std::fmt::Display;

#[derive(Clone)]
pub struct Algorithm {
    moves: Vec<Generator>
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves: Vec<Generator>
            = s.split_whitespace().map(|s| Generator::from_str(s)).collect::<Result<Vec<Generator>, Self::Err>>()?;
        Ok(Algorithm {
            moves: moves
        })
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.moves.iter().map(|g| g.name()).collect::<Vec<String>>().join(" "))
    }
}

impl Algorithm {
    fn length(&self) -> i8 {
        self.moves.len() as i8
    }

    // Gives a vec of the incremental cubestates after each move is applied
    // Used to reconstruct the state of the iterator
    fn cubestates(&self) -> Vec<CubeState> {
        self.moves.iter().scan(CubeState::solved(), |&mut c, next| {
            Some(c.apply(&next.effect))
        }).collect()
    }

    fn rotate(&self) -> Algorithm {
        Algorithm {
            moves: self.moves.iter().map(|m| m.rotate_y()).collect()
        }
    }

    fn score(&self) -> u16 {
        self.moves.iter().map(|m| {
            m.score()
        }).sum()
    }

    pub fn best_rotation(&self) -> Algorithm {
        let mut best_score = self.score();
        let mut best_alg = self.clone();
        let mut current_alg = self.clone();
        for _ in 0..3 {
            current_alg = current_alg.rotate();
            let next_score = current_alg.score();
            if next_score > best_score {
                best_score = next_score;
                best_alg = current_alg.clone();
            }
        }
        best_alg
    }
}

#[test]
fn gives_alg_length() {
    let alg = Algorithm::from_str("R U R' U'").unwrap();
    assert_eq!(alg.length(), 4);
}

#[test]
fn gives_cubestates() {
    let alg = Algorithm::from_str("R U R' U'").unwrap();
    let cubestates = alg.cubestates();
    assert_eq!(cubestates.len(), 4);
}

#[test]
fn can_restringify() {
    let alg = Algorithm::from_str("R U R' U'").unwrap();
    assert_eq!(format!("{}", alg), "R U R' U'");
}

#[test]
fn gives_best_rotation() {
    let alg = Algorithm::from_str("R U R' U'").unwrap();
    let best = alg.best_rotation();
    assert_eq!(format!("{}", best), "R U R' U'");

    let alg2 = Algorithm::from_str("F U F' U'").unwrap();
    let best2 = alg2.best_rotation();
    assert_eq!(format!("{}", best2), "R U R' U'");
}
