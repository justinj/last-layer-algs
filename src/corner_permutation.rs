use prunable::Prunable;

pub const CP_SOLVED: usize = 0;
const NUM_CORNERS: usize = 8;

const FACTORIAL: [u16; 8] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
];

pub type CPIndex = usize;

const NUM_PERMUTATIONS: usize = 40320;

#[derive(Debug, Copy, Clone)]
struct CornerPermutation {
    state: [u8; NUM_CORNERS]
}

const MOVES_BY_INDEX: [CornerPermutation; 18] = [
    CornerPermutation { state: [1, 2, 3, 0, 4, 5, 6, 7] }, // U
    CornerPermutation { state: [2, 3, 0, 1, 4, 5, 6, 7] }, // U2
    CornerPermutation { state: [3, 0, 1, 2, 4, 5, 6, 7] }, // U'

    CornerPermutation { state: [0, 1, 2, 3, 5, 6, 7, 4] }, // D
    CornerPermutation { state: [0, 1, 2, 3, 6, 7, 4, 5] }, // D2
    CornerPermutation { state: [0, 1, 2, 3, 7, 4, 5, 6] }, // D'

    CornerPermutation { state: [3, 1, 2, 5, 0, 4, 6, 7] }, // F
    CornerPermutation { state: [5, 1, 2, 4, 3, 0, 6, 7] }, // F2
    CornerPermutation { state: [4, 1, 2, 0, 5, 3, 6, 7] }, // F'

    CornerPermutation { state: [0, 7, 1, 3, 4, 5, 2, 6] }, // B
    CornerPermutation { state: [0, 6, 7, 3, 4, 5, 1, 2] }, // B2
    CornerPermutation { state: [0, 2, 6, 3, 4, 5, 7, 1] }, // B'

    CornerPermutation { state: [4, 0, 2, 3, 7, 5, 6, 1] }, // R
    CornerPermutation { state: [7, 4, 2, 3, 1, 5, 6, 0] }, // R2
    CornerPermutation { state: [1, 7, 2, 3, 0, 5, 6, 4] }, // R'

    CornerPermutation { state: [0, 1, 6, 2, 4, 3, 5, 7] }, // L
    CornerPermutation { state: [0, 1, 5, 6, 4, 2, 3, 7] }, // L2
    CornerPermutation { state: [0, 1, 3, 5, 4, 6, 2, 7] }, // L'
];

impl CornerPermutation {
    fn new(state: [u8; NUM_CORNERS]) -> Self {
        CornerPermutation {
            state: state
        }
    }

    fn apply(&self, other: &Self) -> Self {
        let mut state: [u8; NUM_CORNERS] = [0; NUM_CORNERS];
        for i in 0..NUM_CORNERS {
            state[i] = self.state[other.state[i] as usize];
        }
        CornerPermutation { state: state }
    }
}

impl Prunable for CornerPermutation {
    fn initial_pos() -> Self {
        CornerPermutation {
            state: [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }

    fn apply_idx(&self, idx: usize) -> Self {
        Self::apply(&self, &MOVES_BY_INDEX[idx])
    }

    fn is_solved(&self) -> bool {
        for i in 4..8 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        true
    }

    fn index(&self) -> usize {
        let mut state = self.state.clone();
        let mut result: usize = 0;
        for i in 0..NUM_CORNERS {
            result += state[i] as usize * FACTORIAL[NUM_CORNERS - i - 1] as usize;
            for j in (i + 1)..NUM_CORNERS {
                if state[j] > state[i] {
                    state[j] -= 1;
                }
            }
        }
        result
    }

    fn total_states() -> usize { NUM_PERMUTATIONS }
}

lazy_static! {
    pub static ref TRANSITIONS: Vec<[usize; 18]> = {
        CornerPermutation::make_transition_table()
    };

    pub static ref PRUNING: Vec<u16> = {
        CornerPermutation::make_pruning_table()
    };
}

#[test]
fn performs_permutation() {
    let perm = CornerPermutation::new([1, 0, 2, 3, 4, 5, 6, 7]);
    let perm2 = CornerPermutation::new([0, 2, 1, 3, 7, 5, 6, 4]);
    let composed = perm.apply(&perm2);
    assert_eq!(vec![1, 2, 0, 3, 7, 5, 6, 4], composed.state);
}

#[test]
fn indexes_permutations() {
    assert_eq!(0, CornerPermutation::new([0, 1, 2, 3, 4, 5, 6, 7]).index());
    assert_eq!(1, CornerPermutation::new([0, 1, 2, 3, 4, 5, 7, 6]).index());
    assert_eq!(40319, CornerPermutation::new([7, 6, 5, 4, 3, 2, 1, 0]).index());
    assert_eq!(5880, CornerPermutation::new([1, 2, 3, 0, 4, 5, 6, 7]).index());
}

#[test]
fn lookup_table() {
    assert_eq!(5880, TRANSITIONS[0][0]);
}

#[test]
fn pruning_table() {
    assert_eq!(0, PRUNING[5880]);
}
