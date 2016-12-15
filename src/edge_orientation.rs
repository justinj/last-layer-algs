use prunable::Prunable;
// TODO: eventually, represent this as a bit array, would make building the tables a bit faster
struct Orientation {
    state: [u8; 12]
}

pub type EOIndex = usize;
pub const EO_SOLVED: EOIndex = 0;
const NUM_ORIENTATIONS: usize = 2048;

#[derive(Debug, Copy, Clone)]
struct OrientationMove {
    positions: [usize; 12],
    orientation_effect: [u8; 12],
}

const MOVES_BY_INDEX: [OrientationMove; 18] = [
    // U
    OrientationMove {
        positions: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // U2
    OrientationMove {
        positions: [2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // U'
    OrientationMove {
        positions: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D
    OrientationMove {
        positions: [0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D2
    OrientationMove {
        positions: [0, 1, 2, 3, 6, 7, 4, 5, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D'
    OrientationMove {
        positions: [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // F
    OrientationMove {
        positions: [9, 1, 2, 3, 8, 5, 6, 7, 0, 4, 10, 11],
        orientation_effect: [1, 0, 0, 0, 1, 0, 0, 0, 1, 1,  0,  0],
    },
    // F2
    OrientationMove {
        positions: [4, 1, 2, 3, 0, 5, 6, 7, 9, 8, 10, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // F'
    OrientationMove {
        positions: [8, 1, 2, 3, 9, 5, 6, 7, 4, 0, 10, 11],
        orientation_effect: [1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0],
    },
    // B
    OrientationMove {
        positions: [0, 1, 10, 3, 4, 5, 11, 7, 8, 9, 6, 2],
        orientation_effect: [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1],
    },
    // B2
    OrientationMove {
        positions: [0, 1, 6, 3, 4, 5, 2, 7, 8, 9, 11, 10],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // B'
    OrientationMove {
        positions: [0, 1, 11, 3, 4, 5, 10, 7, 8, 9, 2, 6],
        orientation_effect: [0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1],
    },
    // R
    OrientationMove {
        positions: [0, 8, 2, 3, 4, 10, 6, 7, 5, 9, 1, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // R2
    OrientationMove {
        positions: [0, 5, 2, 3, 4, 1, 6, 7, 10, 9, 8, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // R'
    OrientationMove {
        positions: [0, 10, 2, 3, 4, 8, 6, 7, 1, 9, 5, 11],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // L
    OrientationMove {
        positions: [0, 1, 2, 11, 4, 5, 6, 9, 8, 3, 10, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // L2
    OrientationMove {
        positions: [0, 1, 2, 7, 4, 5, 6, 3, 8, 11, 10, 9],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
    // L'
    OrientationMove {
        positions: [0, 1, 2, 9, 4, 5, 6, 11, 8, 7, 10, 3],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    },
];

impl Orientation {
    fn apply(&self, b: &OrientationMove) -> Self {
        let mut result = Orientation { state: [0; 12] };
        for i in 0..12 {
            result.state[i] = (self.state[b.positions[i]] + b.orientation_effect[b.positions[i]]) % 2;
        }
        result
    }
}

impl Prunable for Orientation {
    fn initial_pos() -> Self {
        Orientation {
            state: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn apply_idx(&self, idx: usize) -> Self {
        Self::apply(&self, &MOVES_BY_INDEX[idx])
    }

    fn is_solved(&self) -> bool {
        for i in 4..12 {
            if self.state[i] != 0 {
                return false;
            }
        }
        true
    }

    fn index(&self) -> usize {
        let mut result: usize = 0;
        for i in 0..11 {
            result = result * 2 + self.state[i] as usize;
        }
        result
    }

    fn total_states() -> usize { NUM_ORIENTATIONS }
}

lazy_static! {
    pub static ref EO_TRANSITIONS: Vec<[usize; 18]> = {
        Orientation::make_transition_table()
    };

    pub static ref EO_PRUNING: Vec<u16> = {
        Orientation::make_pruning_table()
    };
}
