use generator::{Generator, GENERATORS};
use corner_permutation::{CPIndex, CP_SOLVED, TRANSITIONS, PRUNING};
use corner_orientation::{COIndex, CO_SOLVED, CO_TRANSITIONS, CO_PRUNING};
use edge_orientation::{EOIndex, EO_SOLVED, EO_TRANSITIONS, EO_PRUNING};
use edge_permutation::EdgePermutation;
use generator::move_indices::{F, R, U, RPRIME, UPRIME, FPRIME, D};
use cubestate::CubeState;

#[derive(Debug, Clone, Copy)]
pub struct F2LCubeState {
    pub cubestate: CubeState,
    pub cornerperm: CPIndex,
    pub cornerorie: COIndex,
    pub edgeorie: EOIndex,
    pub edgeperm: EdgePermutation,
}

impl F2LCubeState {
    pub fn new() -> Self {
        F2LCubeState {
            cubestate: CubeState::solved(),
            cornerperm: CP_SOLVED,
            cornerorie: CO_SOLVED,
            edgeorie: EO_SOLVED,
            edgeperm: EdgePermutation::new(),
        }
    }

    // TODO: have a typedef for the move indices?
    fn apply_idx(&self, idx: usize) -> Self {
        F2LCubeState {
            cubestate: self.cubestate.apply(&GENERATORS[idx].effect),
            cornerperm: TRANSITIONS[self.cornerperm][GENERATORS[idx].index()] as CPIndex,
            cornerorie: CO_TRANSITIONS[self.cornerorie][GENERATORS[idx].index()] as COIndex,
            edgeorie: EO_TRANSITIONS[self.edgeorie][GENERATORS[idx].index()] as EOIndex,
            edgeperm: self.edgeperm.apply_idx(idx),
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
        dest.edgeorie = EO_TRANSITIONS[self.edgeorie][GENERATORS[g.index()].index()] as EOIndex;
        self.edgeperm.apply_into_idx(g.index(), &mut dest.edgeperm);
    }

    fn is_id(&self) -> bool {
        self.cornerperm == 0
            && self.cornerorie == 0
            && self.edgeorie == 0
            && self.edgeperm.is_solved()
    }

    pub fn is_ll(&self) -> bool {
        PRUNING[self.cornerperm] == 0
            && CO_PRUNING[self.cornerorie] == 0
            && EO_PRUNING[self.edgeorie] == 0
            && self.edgeperm.is_ll()
            && !(self.is_id()
                 || self.apply_idx(0).is_id()
                 || self.apply_idx(1).is_id()
                 || self.apply_idx(2).is_id())
        // self.cubestate.is_ll() &&
        //     !self.cubestate.is_solved() &&
        //     !self.cubestate.apply(&GENERATORS[U].effect).is_solved() &&
        //     !self.cubestate.apply(&GENERATORS[U2].effect).is_solved() &&
        //     !self.cubestate.apply(&GENERATORS[UPRIME].effect).is_solved()
    }

    pub fn prunable(&self, dist: u16) -> bool {
        dist < PRUNING[self.cornerperm]
            || dist < CO_PRUNING[self.cornerorie] as u16
            || dist < EO_PRUNING[self.edgeorie] as u16
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
