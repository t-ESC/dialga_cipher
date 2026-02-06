use crate::dialga::helper::state::*;
use crate::dialga::helper::bitarray::*;

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
    let mut result = 0_u8;

    for j in 0..8 {
        let bit_value = (byte >> (7-j)) & 0b1; // Isolate bit from each position (MSB-first)
        result |= bit_value << (7-PBI[i][j]); // 7- because we are working in MSB-first, index 7 is lowest bit and 0 is highest
    }

    result
}

pub fn permute_bits_inv(byte: u8, i: usize) -> u8 {
    let mut result = 0_u8;

    for j in 0..8 {
        let bit_value = (byte >> (7-j)) & 0b00000001; // Isolate bit from each position (MSB-first)
        result |= bit_value << (7-PBI_INV[i][j]);
        
    }
    result
}

const SB0: [u8;16] = [0xc, 0xa, 0xd, 3, 0xe, 0xb, 0xf, 7, 8, 9, 1, 5, 0, 2, 4, 6]; //4-bit sbox, used in parallel, symmetrical SBOX

pub fn sub_cell(state: &mut State) {
    let i_sbox: [usize; 4] = [0, 3, 2, 1]; // WTF why is this happening???

    for row in 0..4 {
        for col in 0..4 {
            let i = i_sbox[row];
            let mut state_i = state.0[row][col];
            state_i = permute_bits(state_i, i);

            let mut high_bits = state_i >> 4;
            let mut low_bits = state_i & 0b00001111;

            high_bits = SB0[high_bits as usize];
            low_bits = SB0[low_bits as usize];
        
            state_i = (high_bits << 4) + low_bits;
            state_i = permute_bits_inv(state_i, i);

            state.0[row][col] = state_i;
        }
    }
}