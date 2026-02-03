use crate::helper::state::*;
use crate::helper::bitarray::*;

pub fn r_i(state: &mut State, i: usize) { // roundfunction, called r_1 in paper
    sub_cell(state, i);
    byte_permutation(state, i);
    matrix_mul(state, i);
}

pub fn r_i_inv(state: &mut State, i: usize) {
    matrix_mul(state, i);
    byte_permutation_inv(state, i); //only asymmetric function
    sub_cell(state, i);
}

pub fn matrix_mul(state: &mut State, i: usize) { // make state mutalble for now (i think this is the better way, can check AES impl later
    
    /* State column multiplied with Matrix --> self inverse
    * (0 1 1 1)
    * (1 0 1 1)
    * (1 1 0 1)
    * (1 1 1 0)*/

    let pre_mix:State = *state;
    
    state.0[0][i] = pre_mix.0[1][i] ^ pre_mix.0[2][i] ^ pre_mix.0[3][i];
    state.0[1][i] = pre_mix.0[0][i] ^ pre_mix.0[2][i] ^ pre_mix.0[3][i];
    state.0[2][i] = pre_mix.0[0][i] ^ pre_mix.0[1][i] ^ pre_mix.0[3][i];
    state.0[3][i] = pre_mix.0[0][i] ^ pre_mix.0[1][i] ^ pre_mix.0[2][i];
}

const PI: [[u8; 16];4] = [
	[7, 0, 13, 10, 5, 2, 15, 8, 4, 3, 14, 9, 6, 1, 12, 11],
	[13, 0, 10, 7, 11, 6, 12, 1, 2, 15, 5, 8, 4, 9, 3, 14],
	[7, 13, 10, 0, 6, 12, 11, 1, 5, 15, 8, 2, 4, 14, 9, 3],
	[13, 8, 6, 3, 14, 11, 5, 0, 12, 9, 7, 2, 15, 10, 4, 1],
];

const PI_INV:[[u8; 16];4] = [
	[1, 13, 5, 9, 8, 4, 12, 0, 7, 11, 3, 15, 14, 2, 10, 6],
	[1, 7, 8, 14, 12, 10, 5, 3, 11, 13, 2, 4, 6, 0, 15, 9],
	[3, 7, 11, 15, 12, 8, 4, 0, 10, 14, 2, 6, 5, 1, 13, 9],
	[7, 15, 11, 3, 14, 6, 2, 10, 1, 9, 13, 5, 8, 0, 4, 12],
];

pub fn byte_permutation(state: &mut State, i: usize) {
    let pre_perm = *state;
    let i = (i+3)%4; // same as i-1 % 4
    for row in 0..4 {
        for col in 0..4 {
            let pi_i = PI[i];
            let pi_index = (row << 2) + col;

            let state_index = pi_i[pi_index];

            let highbits: u8 = state_index >> 2;
            let lowbits: u8 = state_index & 0b0011;

            state.0[col][row] = pre_perm.0[lowbits as usize][highbits as usize];

        }
    }
}

pub fn byte_permutation_inv(state: &mut State, i: usize) {
    let pre_perm = *state;
    let i = (i+3)%4; // same as i-1 % 4
    for row in 0..4 {
        for col in 0..4 {
            let pi_i = PI_INV[i];
            let pi_index = (row << 2) + col;

            let state_index = pi_i[pi_index];

            let highbits: u8 = state_index >> 2;
            let lowbits: u8 = state_index & 0b0011;

            state.0[col][row] = pre_perm.0[lowbits as usize][highbits as usize];

        }
    }
}



const PBI: [[u8; 8];4] = [
	[4, 1, 6, 3, 0, 5, 2, 7],
	[1, 6, 7, 0, 5, 2, 3, 4],
	[2, 3, 4, 1, 6, 7, 0, 5],
	[7, 4, 1, 2, 3, 0, 5, 6],
];

const PBI_INV: [[u8; 8];4] = [
	[4, 1, 6, 3, 0, 5, 2, 7], //symmetric to Pb0
	[3, 0, 5, 6, 7, 4, 1, 2],
	[6, 3, 0, 1, 2, 7, 4, 5],
	[5, 2, 3, 4, 1, 6, 7, 0],
];

pub fn permute_bits(byte: u8, i: usize) -> u8 {
    let mut permuted_bit_array = BitArray::from(byte);
    let original_bit_array = BitArray::from(byte);

    for j in 0..8 {
        permuted_bit_array.0[PBI[i][j] as usize] = original_bit_array.0[j];
    }
    
    permuted_bit_array.into()
}

pub fn permute_bits_inv(byte: u8, i: usize) -> u8 {
    let mut permuted_bit_array = BitArray::from(byte);
    let original_bit_array = BitArray::from(byte);

    for j in 0..8 {
        permuted_bit_array.0[PBI_INV[i][j] as usize] = original_bit_array.0[j];
    }
    
    permuted_bit_array.into()
}

const SB0: [u8;16] = [0xc, 0xa, 0xd, 3, 0xe, 0xb, 0xf, 7, 8, 9, 1, 5, 0, 2, 4, 6]; //4-bit sbox, used in parallel, symmetrical SBOX


pub fn sub_cell(state: &mut State, i:usize) {
    for col in 0..4 {
        for row in 0..4 {
            let mut  state_i = state.0[col][row];
            state_i = permute_bits(state_i, i);

            let mut high_bits = state_i >> 4;
            let mut low_bits = state_i & 0b00001111;

            high_bits = SB0[high_bits as usize];
            low_bits = SB0[low_bits as usize];

            state_i = (high_bits << 4) + low_bits;
            state_i = permute_bits_inv(state_i, i);
            state.0[col][row] = state_i;
        }
    }
}