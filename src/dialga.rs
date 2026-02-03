use std::{convert::From};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct State(pub [[u8; 4];4]);

impl State {
    pub fn new(data: [[u8; 4];4]) {
        State(data);
    }

    pub fn from_flat(data: [u8; 16]) -> State {
        let mut mat = [[0_u8; 4];4];
        for i in 0..16 {
            let j = i % 4;
            let k = i / 4;
            mat[j][k] = data[i]; 
        }
        State(mat)
    }
}

impl From<[u8; 16]> for State {
    fn from(value: [u8; 16]) -> Self {
        State::from_flat(value)
    }
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitArray ([bool; 8]); // ASSUMPTION: Most Significant Bit First

impl From<u8> for BitArray {
    fn from(value: u8) -> Self {
        let mut output = [false; 8];
        for j in 0..8 {
            if value&(1<<j) > 0 { // First Element here is least significant bit
                output[7-j] = true;
            }
        }
        BitArray(output)
    }
}

impl Into<u8> for BitArray {
    fn into(self) -> u8 {
        let mut output = 0u8;
        for j in 0..8 {
            if self.0[j] {
                output += 1 << (7-j);
            }
        }
        return output;
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