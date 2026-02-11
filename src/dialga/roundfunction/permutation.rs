use crate::dialga::helper::state::*;

const PI: [[usize; 16];4] = [
	[7, 0, 13, 10, 5, 2, 15, 8, 4, 3, 14, 9, 6, 1, 12, 11],
	[13, 0, 10, 7, 11, 6, 12, 1, 2, 15, 5, 8, 4, 9, 3, 14],
	[7, 13, 10, 0, 6, 12, 11, 1, 5, 15, 8, 2, 4, 14, 9, 3],
	[13, 8, 6, 3, 14, 11, 5, 0, 12, 9, 7, 2, 15, 10, 4, 1],
];

const PI_INV:[[usize; 16];4] = [
	[1, 13, 5, 9, 8, 4, 12, 0, 7, 11, 3, 15, 14, 2, 10, 6],
	[1, 7, 8, 14, 12, 10, 5, 3, 11, 13, 2, 4, 6, 0, 15, 9],
	[3, 7, 11, 15, 12, 8, 4, 0, 10, 14, 2, 6, 5, 1, 13, 9],
	[7, 15, 11, 3, 14, 6, 2, 10, 1, 9, 13, 5, 8, 0, 4, 12],
];

pub fn byte_permutation(state: &mut State, r: usize) -> State {
    let pre_perm = *state;
    let r = (r+3)%4; // same as i-1 % 4
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI[r][idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[pi_row][pi_col] = pre_perm.0[row][col];
        }
    }
    *state
}

pub fn byte_permutation_inv(state: &mut State, r: usize) -> State {
    let pre_perm = *state;
    let r = (r+3)%4; // same as i-1 % 4
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI_INV[r][idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[pi_row][pi_col] = pre_perm.0[row][col];
        }
    }
    *state
}