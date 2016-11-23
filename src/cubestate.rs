use std::fmt;

const NUM_STICKERS: usize = 54;

//          ┌──┬──┬──┐
//          │ 0│ 1│ 2│
//          ├──┼──┼──┤
//          │ 3│ 4│ 5│
//          ├──┼──┼──┤
//          │ 6│ 7│ 8│
// ┌──┬──┬──┼──┼──┼──┼──┬──┬──┐
// │ 9│10│11│12│13│14│15│16│17│
// ├──┼──┼──┼──┼──┼──┼──┼──┼──┤
// │18│19│20│21│22│23│24│25│26│
// ├──┼──┼──┼──┼──┼──┼──┼──┼──┤
// │27│28│29│30│31│32│33│34│35│
// └──┴──┴──┼──┼──┼──┼──┴──┴──┘
//          │36│37│38│
//          ├──┼──┼──┤
//          │39│40│41│
//          ├──┼──┼──┤
//          │42│43│44│
//          ├──┼──┼──┤
//          │45│46│47│
//          ├──┼──┼──┤
//          │48│49│50│
//          ├──┼──┼──┤
//          │51│52│53│
//          └──┴──┴──┘

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
        let mut dest = Self::solved();
        Self::apply_into(&self, other, &mut dest);
        dest
    }

    // Apply `other` to `self`, storing the result in `dest`.
    // This exists in order to be able to avoid an allocation.
    pub fn apply_into(&self, other: &CubeState, dest: &mut CubeState) {
        for i in 0..NUM_STICKERS {
            dest.state[i] = self.state[(*other).state[i] as usize];
        }
    }

    // That this is as simple as it is just arises from the way we represent a cube
    pub fn is_ll(self) -> bool {
        for i in 18..51 {
            if self.state[i] != i as u8 {
                return false;
            }
        }
        return true;
    }
}
