use std::collections::VecDeque;

pub trait Prunable : Sized {
    fn initial_pos() -> Self;
    fn apply_idx(&self, idx: usize) -> Self;
    fn is_solved(&self) -> bool;
    fn index(&self) -> usize;
    fn total_states() -> usize;

    fn make_transition_table() -> Vec<[usize; 18]> {
        let mut result = vec![];
        let mut visited: Vec<bool> = vec![];
        for _ in 0..Self::total_states() {
            result.push([0; 18]);
            visited.push(false);
        }

        let mut stack = vec![Self::initial_pos()];
        while let Some(state) = stack.pop() {
            let idx = state.index();
            if !visited[idx as usize] {
                visited[idx as usize] = true;
                for i in 0..18 {
                    let new_state = state.apply_idx(i);
                    result[idx as usize][i] = new_state.index();
                    stack.push(new_state);
                }
            }
        }
        result
    }

    fn make_pruning_table() -> Vec<u16> {
        let mut result = vec![];
        for _ in 0..Self::total_states() {
            result.push(100);
        }

        let mut queue = VecDeque::new();
        queue.push_back((0, Self::initial_pos()));
        while let Some((distance, state)) = queue.pop_front() {
            let idx = state.index();
            if distance < result[idx as usize] {
                result[idx as usize] = distance;
                for i in 0..18 {
                    let new_state = state.apply_idx(i);
                    if new_state.is_solved() {
                        queue.push_back((0, new_state));
                    } else {
                        queue.push_back((distance + 1, new_state));
                    }
                }
            }
        }
        result
    }
}
