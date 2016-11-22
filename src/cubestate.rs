#![allow(dead_code)]
const NUM_STICKERS: usize = 54;

use std::fmt;

#[derive(Copy)]
pub struct CubeState {
    pub state: [u8; NUM_STICKERS],
}

impl fmt::Debug for CubeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.state.iter().map(|x| format!("{:?}", x)).collect::<Vec<String>>().join(", "))
    }
}

impl Clone for CubeState {
    fn clone(&self) -> CubeState {
        let mut new = CubeState { state: [0; NUM_STICKERS] };
        for i in 0..NUM_STICKERS {
            new.state[i] = self.state[i];
        }
        new
    }
}

impl CubeState {
    pub fn solved() -> CubeState {
        let mut stickers: [u8; NUM_STICKERS] = [0; NUM_STICKERS];
        for i in 0..NUM_STICKERS {
            stickers[i] = i as u8;
        }
        CubeState { state: stickers }
    }

    pub fn apply(&self, other: &CubeState) -> CubeState {
        let mut stickers: [u8; NUM_STICKERS] = [0; NUM_STICKERS];
        for i in 0..NUM_STICKERS {
            stickers[i] = self.state[(*other).state[i] as usize];
        }
        CubeState { state: stickers }
    }

    pub fn apply_into(&self, other: &CubeState, dest: &mut CubeState) {
        for i in 0..NUM_STICKERS {
            dest.state[i] = self.state[(*other).state[i] as usize];
        }
    }


    pub fn is_ll(self) -> bool {
        for i in 18..51 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        return true;
    }
}
