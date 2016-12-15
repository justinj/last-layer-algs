#[derive(Debug, Copy, Clone)]
pub struct EdgePermutation {
    state: [u8; 12]
}

const MOVES_BY_INDEX: [EdgePermutation; 18] = [
    EdgePermutation { state: [1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11] }, // U
    EdgePermutation { state: [2, 3, 0, 1, 4, 5, 6, 7, 8, 9, 10, 11] }, // U2
    EdgePermutation { state: [3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11] }, // U'
    EdgePermutation { state: [0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11] }, // D
    EdgePermutation { state: [0, 1, 2, 3, 6, 7, 4, 5, 8, 9, 10, 11] }, // D2
    EdgePermutation { state: [0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11] }, // D'
    EdgePermutation { state: [9, 1, 2, 3, 8, 5, 6, 7, 0, 4, 10, 11] }, // F
    EdgePermutation { state: [4, 1, 2, 3, 0, 5, 6, 7, 9, 8, 10, 11] }, // F2
    EdgePermutation { state: [8, 1, 2, 3, 9, 5, 6, 7, 4, 0, 10, 11] }, // F'
    EdgePermutation { state: [0, 1, 10, 3, 4, 5, 11, 7, 8, 9, 6, 2] }, // B
    EdgePermutation { state: [0, 1, 6, 3, 4, 5, 2, 7, 8, 9, 11, 10] }, // B2
    EdgePermutation { state: [0, 1, 11, 3, 4, 5, 10, 7, 8, 9, 2, 6] }, // B'
    EdgePermutation { state: [0, 8, 2, 3, 4, 10, 6, 7, 5, 9, 1, 11] }, // R
    EdgePermutation { state: [0, 5, 2, 3, 4, 1, 6, 7, 10, 9, 8, 11] }, // R2
    EdgePermutation { state: [0, 10, 2, 3, 4, 8, 6, 7, 1, 9, 5, 11] }, // R'
    EdgePermutation { state: [0, 1, 2, 11, 4, 5, 6, 9, 8, 3, 10, 7] }, // L
    EdgePermutation { state: [0, 1, 2, 7, 4, 5, 6, 3, 8, 11, 10, 9] }, // L2
    EdgePermutation { state: [0, 1, 2, 9, 4, 5, 6, 11, 8, 7, 10, 3] }, // L'
];

impl EdgePermutation {
    pub fn new() -> Self {
        EdgePermutation {
            state: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        }
    }

    pub fn is_ll(&self) -> bool {
        for i in 4..12 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        true
    }

    pub fn is_solved(&self) -> bool {
        for i in 0..12 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        true
    }

    pub fn apply_idx(&self, idx: usize) -> Self {
        let mut dest = EdgePermutation::new();
        self.apply_into_idx(idx, &mut dest);
        dest
    }

    pub fn apply_into_idx(&self, idx: usize, dest: &mut EdgePermutation) {
        let other = &MOVES_BY_INDEX[idx];
        for i in 0..12 {
            dest.state[i] = self.state[other.state[i] as usize];
        }
    }
}
