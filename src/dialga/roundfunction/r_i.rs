use crate::dialga::helper::state::State;
use crate::dialga::roundfunction::matrix_mul::*;
use crate::dialga::roundfunction::sub_cell::*;
use crate::dialga::roundfunction::permutation::*;

pub fn r_i(state: &mut State, i: usize) -> State { // roundfunction, called r_1 in paper
    sub_cell_inv(state);
    byte_permutation(state, i);
    matrix_mul(state);
    *state
}

pub fn r_i_inv(state: &mut State, i: usize) -> State {
    matrix_mul(state);
    byte_permutation_inv(state, i); //only asymmetric function
    sub_cell_inv(state);
    *state
}
