use prunable::Prunable;

struct Orientation {
    state: [u8; 8],
}

pub type COIndex = usize;
pub const CO_SOLVED: COIndex = 0;
const NUM_ORIENTATIONS: usize = 2187;

#[derive(Debug, Copy, Clone)]
struct OrientationMove {
    positions: [usize; 8],
    orientation_effect: [u8; 8],
}

const MOVES_BY_INDEX: [OrientationMove; 18] = [
    // U
    OrientationMove {
        positions: [1, 2, 3, 0, 4, 5, 6, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // U2
    OrientationMove {
        positions: [2, 3, 0, 1, 4, 5, 6, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // U'
    OrientationMove {
        positions: [3, 0, 1, 2, 4, 5, 6, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D
    OrientationMove {
        positions: [0, 1, 2, 3, 5, 6, 7, 4],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D2
    OrientationMove {
        positions: [0, 1, 2, 3, 6, 7, 4, 5],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // D'
    OrientationMove {
        positions: [0, 1, 2, 3, 7, 4, 5, 6],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // F
    OrientationMove {
        positions: [3, 1, 2, 5, 0, 4, 6, 7],
        orientation_effect: [1, 0, 0, 2, 2, 1, 0, 0],
    },
    // F2
    OrientationMove {
        positions: [5, 1, 2, 4, 3, 0, 6, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // F'
    OrientationMove {
        positions: [4, 1, 2, 0, 5, 3, 6, 7],
        orientation_effect: [1, 0, 0, 2, 2, 1, 0, 0],
    },
    // B
    OrientationMove {
        positions: [0, 7, 1, 3, 4, 5, 2, 6],
        orientation_effect: [0, 2, 1, 0, 0, 0, 2, 1],
    },
    // B2
    OrientationMove {
        positions: [0, 6, 7, 3, 4, 5, 1, 2],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // B'
    OrientationMove {
        positions: [0, 2, 6, 3, 4, 5, 7, 1],
        orientation_effect: [0, 2, 1, 0, 0, 0, 2, 1],
    },
    // R
    OrientationMove {
        positions: [4, 0, 2, 3, 7, 5, 6, 1],
        orientation_effect: [2, 1, 0, 0, 1, 0, 0, 2],
    },
    // R2
    OrientationMove {
        positions: [7, 4, 2, 3, 1, 5, 6, 0],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // R'
    OrientationMove {
        positions: [1, 7, 2, 3, 0, 5, 6, 4],
        orientation_effect: [2, 1, 0, 0, 1, 0, 0, 2],
    },
    // L
    OrientationMove {
        positions: [0, 1, 6, 2, 4, 3, 5, 7],
        orientation_effect: [0, 0, 2, 1, 0, 2, 1, 0],
    },
    // L2
    OrientationMove {
        positions: [0, 1, 5, 6, 4, 2, 3, 7],
        orientation_effect: [0, 0, 0, 0, 0, 0, 0, 0],
    },
    // L'
    OrientationMove {
        positions: [0, 1, 3, 5, 4, 6, 2, 7],
        orientation_effect: [0, 0, 2, 1, 0, 2, 1, 0],
    },
];

impl Orientation {
    fn apply(&self, b: &OrientationMove) -> Self {
        let mut result = Orientation { state: [0; 8] };
        for i in 0..8 {
            result.state[i] = (self.state[b.positions[i]] + b.orientation_effect[i]) % 3;
        }
        result
    }
}

impl Prunable for Orientation {
    fn initial_pos() -> Self {
        Orientation {
            state: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }

    fn apply_idx(&self, idx: usize) -> Self {
        Self::apply(&self, &MOVES_BY_INDEX[idx])
    }

    fn is_solved(&self) -> bool {
        for i in 4..8 {
            if self.state[i] != 0 {
                return false;
            }
        }
        true
    }

    fn index(&self) -> usize {
        let mut result: usize = 0;
        for i in 0..7 {
            result = result * 3 + self.state[i] as usize;
        }
        result
    }

    fn total_states() -> usize { NUM_ORIENTATIONS }
}

// TODO: this is copied from corner_permutation, we should find a nice way to generalize it
lazy_static! {
    pub static ref CO_TRANSITIONS: Vec<[usize; 18]> = {
        Orientation::make_transition_table()
    };

    pub static ref CO_PRUNING: Vec<u16> = {
        Orientation::make_pruning_table()
    };
}


// fn permute_by_move_index(o: Orientation, idx: usize) -> Orientation {
//     let cycle = &CYCLES[idx];
//     let mut result: u16 = 0;
//     for i in 0..8 {
//         result = (result << 2) | (o >> (7 - cycle.positions[i])) & 0b11;
//     }
//     result
// }

// #[test]
// fn permutes_bits() {
//     assert_eq!(0b0001100000000000, permute_by_move_index(0b0000011000000000, 0));
// }

// use test::Bencher;

// #[bench]
// fn int_bench(b: &mut Bencher) {
//     b.iter(|| {
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//         ::test::black_box(add(0b1010100101010000, 0b0110011000100110));
//     });
// }



// #[bench]
// fn array_bench(bench: &mut Bencher) {
//     let mut c = [0, 0, 0, 0, 0, 0, 0, 0];
//     let mut a = [0, 1, 2, 0, 1, 2, 0, 1];
//     let mut b = [1, 0, 2, 2, 2, 1, 0, 1];
//     bench.iter(|| {
//         add_into(a, b, &mut c);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(a, b, &mut c);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         add_into(c, a, &mut b);
//         add_into(b, c, &mut a);
//         ::test::black_box(c);
//         ::test::black_box(a);
//         ::test::black_box(b);
//     });
// }
// type Orientation = u16;

// const FIRSTS : u16 = 0b1010101010101010;
// const SECONDS: u16 = 0b0101010101010101;
// fn add(a: Orientation, b: Orientation) -> Orientation {
//     let anya = a & SECONDS | ((a & FIRSTS) >> 1);
//     let anyb = b & SECONDS | ((b & FIRSTS) >> 1);
//     let ones = (FIRSTS  & a & b) >> 1 | (((a & SECONDS) ^ (b & SECONDS)) & (anya ^ anyb));
//     let twos = (SECONDS & a & b) << 1 | (((a & FIRSTS)  ^ (b & FIRSTS))  & (anya ^ anyb) << 1);
//     ones | twos
// }

// #[test]
// fn performs_mod3_addition() {
//     assert_eq!(0b0000, add(0b0000, 0b0000));
//     assert_eq!(0b0000, add(0b0001, 0b0010));
//     assert_eq!(0b0000, add(0b0010, 0b0001));

//     assert_eq!(0b0001, add(0b0010, 0b0010));
//     assert_eq!(0b0001, add(0b0001, 0b0000));
//     assert_eq!(0b0001, add(0b0000, 0b0001));

//     assert_eq!(0b0010, add(0b0001, 0b0001));
//     assert_eq!(0b0010, add(0b0000, 0b0010));
//     assert_eq!(0b0010, add(0b0010, 0b0000));
// }
