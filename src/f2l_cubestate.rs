use generator::{Generator, GENERATORS};
use corner_permutation::{CPIndex, CP_SOLVED, TRANSITIONS, PRUNING};
use corner_orientation::{COIndex, CO_SOLVED, CO_TRANSITIONS, CO_PRUNING};
use generator::move_indices::{F, R, U, RPRIME, UPRIME, U2, FPRIME, D};
use cubestate::CubeState;

#[derive(Debug, Clone, Copy)]
pub struct F2LCubeState {
    pub cubestate: CubeState,
    pub cornerperm: CPIndex,
    pub cornerorie: COIndex,
}

impl F2LCubeState {
    pub fn new() -> Self {
        F2LCubeState {
            cubestate: CubeState::solved(),
            cornerperm: CP_SOLVED,
            cornerorie: CO_SOLVED,
        }
    }

    // TODO: have a typedef for the move indices?
    fn apply_idx(&self, idx: usize) -> Self {
        F2LCubeState {
            cubestate: self.cubestate.apply(&GENERATORS[idx].effect),
            cornerperm: TRANSITIONS[self.cornerperm][GENERATORS[idx].index()] as CPIndex,
            cornerorie: CO_TRANSITIONS[self.cornerorie][GENERATORS[idx].index()] as COIndex,
        }
    }

    pub fn apply(&self, g: Generator) -> F2LCubeState {
        let mut result = F2LCubeState::new();
        self.apply_into(g, &mut result);
        result
    }

    pub fn apply_into(&self, g: Generator, dest: &mut Self) {
        self.cubestate.apply_into(&g.effect, &mut dest.cubestate);
        dest.cornerperm = TRANSITIONS[self.cornerperm][GENERATORS[g.index()].index()] as CPIndex;
        dest.cornerorie = CO_TRANSITIONS[self.cornerorie][GENERATORS[g.index()].index()] as COIndex;
    }

    pub fn is_ll(&self) -> bool {
        self.cubestate.is_ll() &&
            !self.cubestate.is_solved() &&
            !self.cubestate.apply(&GENERATORS[U].effect).is_solved() &&
            !self.cubestate.apply(&GENERATORS[U2].effect).is_solved() &&
            !self.cubestate.apply(&GENERATORS[UPRIME].effect).is_solved()
    }

    pub fn prunable(&self, dist: u16) -> bool {
        dist < PRUNING[self.cornerperm] || dist < CO_PRUNING[self.cornerorie] as u16
    }
}

#[test]
fn is_ll_tells_if_state_is_ll() {
    let state = F2LCubeState::new();
    let result = state.apply_idx(D);
    assert!(!result.is_ll());

    let result2 =
        state.apply_idx(F)
             .apply_idx(R)
             .apply_idx(U)
             .apply_idx(RPRIME)
             .apply_idx(UPRIME)
             .apply_idx(FPRIME);
    assert!(result2.is_ll());
}

#[test]
fn is_ll_is_false_if_ll_is_solved() {
    let state = F2LCubeState::new();
    assert!(!state.is_ll());

    let result = state.apply_idx(U);
    assert!(!result.is_ll());
}
