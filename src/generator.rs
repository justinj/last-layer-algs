use std::str::FromStr;
use ::cubestate::CubeState as CubeState;
use ::std::fmt::Display;
use ::lla_error::LLAError;
use self::LLAError::InvalidAlgorithm;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Face {
    U, D, F, B, R, L
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Axis {
    UD, FB, RL
}

impl Face {
    // Used to pick a "best" rotation of an algorithm to present.
    pub fn score(&self) -> u16 {
        match *self {
            Face::U => 0,
            Face::D => 0,
            Face::F => 2,
            Face::B => 0,
            Face::R => 3,
            Face::L => 1,
        }
    }

    fn rotate_y(&self) -> Face {
        match self {
            &Face::U => Face::U,
            &Face::D => Face::D,
            &Face::F => Face::L,
            &Face::B => Face::R,
            &Face::R => Face::F,
            &Face::L => Face::B,
        }
    }

    fn axis(&self) -> Axis {
        match self {
            &Face::U => Axis::UD,
            &Face::D => Axis::UD,
            &Face::F => Axis::FB,
            &Face::B => Axis::FB,
            &Face::R => Axis::RL,
            &Face::L => Axis::RL,
        }
    }

    fn is_primary(&self) -> bool {
        match self {
            &Face::U => true,
            &Face::F => true,
            &Face::R => true,
            _ => false,
        }
    }
}

impl Display for Face {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", match self {
            &Face::U => "U",
            &Face::D => "D",
            &Face::F => "F",
            &Face::B => "B",
            &Face::R => "R",
            &Face::L => "L",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Modifier {
    Normal, Twice, Prime
}

impl Display for Modifier {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", match self {
            &Modifier::Normal => "",
            &Modifier::Twice => "2",
            &Modifier::Prime => "'",
        })
    }
}

impl Modifier {
    fn inverse(&self) -> Self {
        match self {
            &Modifier::Normal => Modifier::Prime,
            &Modifier::Twice  => Modifier::Twice,
            &Modifier::Prime  => Modifier::Normal,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Generator {
    pub effect: CubeState,
    pub face: Face,
    modifier: Modifier,
}

pub static GENERATORS: [Generator; 18] = [
    Generator { face: Face::U, modifier: Modifier::Normal, effect: CubeState { state: [6,3,0,7,4,1,8,5,2,12,13,14,15,16,17,53,52,51,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,11,10,9] } },
    Generator { face: Face::U, modifier: Modifier::Twice,  effect: CubeState { state: [8,7,6,5,4,3,2,1,0,15,16,17,53,52,51,9,10,11,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,14,13,12] } },
    Generator { face: Face::U, modifier: Modifier::Prime,  effect: CubeState { state: [2,5,8,1,4,7,0,3,6,53,52,51,9,10,11,12,13,14,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,17,16,15] } },
    Generator { face: Face::D, modifier: Modifier::Normal, effect: CubeState { state: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,47,46,45,27,28,29,30,31,32,42,39,36,43,40,37,44,41,38,35,34,33,48,49,50,51,52,53] } },
    Generator { face: Face::D, modifier: Modifier::Twice,  effect: CubeState { state: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,33,34,35,47,46,45,27,28,29,44,43,42,41,40,39,38,37,36,32,31,30,48,49,50,51,52,53] } },
    Generator { face: Face::D, modifier: Modifier::Prime,  effect: CubeState { state: [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,30,31,32,33,34,35,47,46,45,38,41,44,37,40,43,36,39,42,29,28,27,48,49,50,51,52,53] } },
    Generator { face: Face::F, modifier: Modifier::Normal, effect: CubeState { state: [0,1,2,3,4,5,29,20,11,9,10,36,30,21,12,6,16,17,18,19,37,31,22,13,7,25,26,27,28,38,32,23,14,8,34,35,33,24,15,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53] } },
    Generator { face: Face::F, modifier: Modifier::Twice,  effect: CubeState { state: [0,1,2,3,4,5,38,37,36,9,10,33,32,31,30,29,16,17,18,19,24,23,22,21,20,25,26,27,28,15,14,13,12,11,34,35,8,7,6,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53] } },
    Generator { face: Face::F, modifier: Modifier::Prime,  effect: CubeState { state: [0,1,2,3,4,5,15,24,33,9,10,8,14,23,32,38,16,17,18,19,7,13,22,31,37,25,26,27,28,6,12,21,30,36,34,35,11,20,29,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53] } },
    Generator { face: Face::B, modifier: Modifier::Normal, effect: CubeState { state: [17,26,35,3,4,5,6,7,8,2,10,11,12,13,14,15,16,44,1,19,20,21,22,23,24,25,43,0,28,29,30,31,32,33,34,42,36,37,38,39,40,41,9,18,27,51,48,45,52,49,46,53,50,47] } },
    Generator { face: Face::B, modifier: Modifier::Twice,  effect: CubeState { state: [44,43,42,3,4,5,6,7,8,35,10,11,12,13,14,15,16,27,26,19,20,21,22,23,24,25,18,17,28,29,30,31,32,33,34,9,36,37,38,39,40,41,2,1,0,53,52,51,50,49,48,47,46,45] } },
    Generator { face: Face::B, modifier: Modifier::Prime,  effect: CubeState { state: [27,18,9,3,4,5,6,7,8,42,10,11,12,13,14,15,16,0,43,19,20,21,22,23,24,25,1,44,28,29,30,31,32,33,34,2,36,37,38,39,40,41,35,26,17,47,50,53,46,49,52,45,48,51] } },
    Generator { face: Face::R, modifier: Modifier::Normal, effect: CubeState { state: [0,1,14,3,4,23,6,7,32,9,10,11,12,13,38,33,24,15,18,19,20,21,22,41,34,25,16,27,28,29,30,31,44,35,26,17,36,37,47,39,40,50,42,43,53,45,46,2,48,49,5,51,52,8] } },
    Generator { face: Face::R, modifier: Modifier::Twice,  effect: CubeState { state: [0,1,38,3,4,41,6,7,44,9,10,11,12,13,47,35,34,33,18,19,20,21,22,50,26,25,24,27,28,29,30,31,53,17,16,15,36,37,2,39,40,5,42,43,8,45,46,14,48,49,23,51,52,32] } },
    Generator { face: Face::R, modifier: Modifier::Prime,  effect: CubeState { state: [0,1,47,3,4,50,6,7,53,9,10,11,12,13,2,17,26,35,18,19,20,21,22,5,16,25,34,27,28,29,30,31,8,15,24,33,36,37,14,39,40,23,42,43,32,45,46,38,48,49,41,51,52,44] } },
    Generator { face: Face::L, modifier: Modifier::Normal, effect: CubeState { state: [45,1,2,48,4,5,51,7,8,27,18,9,0,13,14,15,16,17,28,19,10,3,22,23,24,25,26,29,20,11,6,31,32,33,34,35,12,37,38,21,40,41,30,43,44,36,46,47,39,49,50,42,52,53] } },
    Generator { face: Face::L, modifier: Modifier::Twice,  effect: CubeState { state: [36,1,2,39,4,5,42,7,8,29,28,27,45,13,14,15,16,17,20,19,18,48,22,23,24,25,26,11,10,9,51,31,32,33,34,35,0,37,38,3,40,41,6,43,44,12,46,47,21,49,50,30,52,53] } },
    Generator { face: Face::L, modifier: Modifier::Prime,  effect: CubeState { state: [12,1,2,21,4,5,30,7,8,11,20,29,36,13,14,15,16,17,10,19,28,39,22,23,24,25,26,9,18,27,42,31,32,33,34,35,45,37,38,48,40,41,51,43,44,0,46,47,3,49,50,6,52,53] } },
];

lazy_static! {
    static ref GENERATOR_SUCCESSORS: Vec<Vec<&'static Generator>> = {
        GENERATORS.iter().map(|g| {
            g.successors_()
        }).collect()
    };
}

// TODO: replace that usage of fromstr with one of these
#[allow(dead_code)]
pub mod move_indices {
    pub const U      : usize = 0;
    pub const U2     : usize = 1;
    pub const UPRIME : usize = 2;
    pub const D      : usize = 3;
    pub const D2     : usize = 4;
    pub const DPRIME : usize = 5;
    pub const F      : usize = 6;
    pub const F2     : usize = 7;
    pub const FPRIME : usize = 8;
    pub const B      : usize = 9;
    pub const B2     : usize = 10;
    pub const BPRIME : usize = 11;
    pub const R      : usize = 12;
    pub const R2     : usize = 13;
    pub const RPRIME : usize = 14;
    pub const L      : usize = 15;
    pub const L2     : usize = 16;
    pub const LPRIME : usize = 17;
}

impl FromStr for Generator {
    type Err = LLAError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let idx = match s {
            "U"  => Ok(move_indices::U),
            "U2" => Ok(move_indices::U2),
            "U'" => Ok(move_indices::UPRIME),
            "D"  => Ok(move_indices::D),
            "D2" => Ok(move_indices::D2),
            "D'" => Ok(move_indices::DPRIME),
            "F"  => Ok(move_indices::F),
            "F2" => Ok(move_indices::F2),
            "F'" => Ok(move_indices::FPRIME),
            "B"  => Ok(move_indices::B),
            "B2" => Ok(move_indices::B2),
            "B'" => Ok(move_indices::BPRIME),
            "R"  => Ok(move_indices::R),
            "R2" => Ok(move_indices::R2),
            "R'" => Ok(move_indices::RPRIME),
            "L"  => Ok(move_indices::L),
            "L2" => Ok(move_indices::L2),
            "L'" => Ok(move_indices::LPRIME),
            &_ => Err(InvalidAlgorithm(format!("No move '{}'", s)))
        }?;
        Ok(GENERATORS[idx])
    }
}

impl Display for Generator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}{}", self.face, self.modifier)
    }
}

impl Generator {
    pub fn first() -> &'static Generator {
        &GENERATORS[12]
    }

    pub fn starting_moves() -> Vec<&'static Generator> {
        vec![&GENERATORS[12], &GENERATORS[13], &GENERATORS[14]]
    }

    pub fn is_u_move(&self) -> bool {
        self.face == Face::U
    }

    pub fn is_d_move(&self) -> bool {
        self.face == Face::D
    }

    pub fn inverse(&self) -> Self {
        Self::from_face_and_modifier(self.face, self.modifier.inverse())
    }

    pub fn rotate_y(&self) -> Self {
        Self::from_face_and_modifier(self.face.rotate_y(), self.modifier)
    }

    fn from_face_and_modifier(f: Face, m: Modifier) -> Self {
        GENERATORS[f as usize * 3 + m as usize]
    }

    pub fn score(&self) -> u16 {
        self.face.score()
    }

    pub fn components(&self) -> (Face, Modifier) {
        (self.face, self.modifier)
    }

    // the moves which can follow another move are those which
    // * are on a different axis from the given move OR
    // * are on the same axis but a different face IF the given move is on U, F, or R.
    fn successors_(&self) -> Vec<&'static Generator> {
        GENERATORS.iter().filter(|g| self.is_valid_successor(g)).collect()
    }

    pub fn is_valid_successor(&self, g: &Generator) -> bool {
        &g.face.axis() != &self.face.axis()
        || &g.face != &self.face && self.face.is_primary()
    }

    pub fn successors(&self) -> &'static Vec<&'static Generator> {
        &GENERATOR_SUCCESSORS[self.index()]
    }

    pub fn index(&self) -> usize {
        self.face as usize * 3 + self.modifier as usize
    }
}

#[test]
fn gives_successors() {
    let f = Generator::from_str("F").unwrap();
    assert_eq!(
        f.successors().iter().map(|g| format!("{}", g)).collect::<Vec<String>>().join(" "),
        "U U2 U' D D2 D' B B2 B' R R2 R' L L2 L'"
    );
}
