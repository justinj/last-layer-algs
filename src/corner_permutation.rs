#![allow(dead_code)]
use std::collections::VecDeque;

pub const SOLVED: usize = 0;
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

    fn apply_idx(&self, idx: usize) -> Self {
        Self::apply(&self, &MOVES_BY_INDEX[idx])
    }

    fn is_fl_solved(&self) -> bool {
        for i in 4..8 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        true
    }

    fn index(&self) -> u16 {
        let mut state = self.state.clone();
        let mut result: u16 = 0;
        for i in 0..NUM_CORNERS {
            result += state[i] as u16 * FACTORIAL[NUM_CORNERS - i - 1];
            for j in (i + 1)..NUM_CORNERS {
                if state[j] > state[i] {
                    state[j] -= 1;
                }
            }
        }
        result
    }
}

lazy_static! {
    pub static ref TRANSITIONS: Vec<[u16; 18]> = {
        let mut result = vec![];
        let mut visited: Vec<bool> = vec![];
        for _ in 0..NUM_PERMUTATIONS {
            result.push([0; 18]);
            visited.push(false);
        }

        let mut stack = vec![CornerPermutation::new([0, 1, 2, 3, 4, 5, 6, 7])];
        while let Some(state) = stack.pop() {
            let idx = state.index();
            if !visited[idx as usize] {
                visited[idx as usize] = true;
                for i in 0..18 {
                    let new_state = state.apply_idx(i);
                    stack.push(new_state);
                    result[idx as usize][i] = new_state.index();
                }
            }
        }
        result
    };

    pub static ref PRUNING: Vec<u16> = {
        let mut result = vec![];
        for _ in 0..NUM_PERMUTATIONS {
            result.push(100);
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, CornerPermutation::new([0, 1, 2, 3, 4, 5, 6, 7])));
        while let Some((distance, state)) = queue.pop_front() {
            let idx = state.index();
            if distance < result[idx as usize] {
                result[idx as usize] = distance;
                for i in 0..18 {
                    if state.is_fl_solved() {
                        queue.push_back((0, state.apply_idx(i)));
                    } else {
                        queue.push_back((distance + 1, state.apply_idx(i)));
                    }
                }
            }
        }
        result
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
