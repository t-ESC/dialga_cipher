use crate::dialga::helper::state::State;

pub const PI_M: [u8; 16] = [0, 0xa, 5, 0xf, 0xe, 4, 0xb, 1, 9, 3, 0xc, 6, 7, 0xd, 2, 8];
pub const PI_M_INV: [u8; 16] = [0, 7, 0xe, 9, 5, 2, 0xb, 0xc, 0xf, 8, 1, 6, 0xa, 0xd, 4, 3];

pub fn ms(state: &mut State) {
    for row in 0..4 {
        for col in 0..4 {
            let mut high_bits = state.0[row][col] >> 4;
            let mut low_bits = state.0[row][col] & 0b00001111;

            high_bits = PI_M[high_bits as usize];
            low_bits = PI_M[low_bits as usize];

            state.0[row][col] = (high_bits << 4) + low_bits; 
        }
    }
}

pub fn ms_inv(state: &mut State) {
    for row in 0..4 {
        for col in 0..4 {
            let mut high_bits = state.0[row][col] >> 4;
            let mut low_bits = state.0[row][col] & 0b00001111;
            
            high_bits = PI_M_INV[high_bits as usize];
            low_bits = PI_M_INV[low_bits as usize];

            state.0[row][col] = (high_bits << 4) + low_bits;        
        }
    }
}