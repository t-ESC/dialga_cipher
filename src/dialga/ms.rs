use crate::dialga::helper::state::State;

pub const PI_M: [u8; 16] = [0, 0xa, 5, 0xf, 0xe, 4, 0xb, 1, 9, 3, 0xc, 6, 7, 0xd, 2, 8];
pub const PI_M_INV: [u8; 16] = [0, 7, 0xe, 9, 5, 2, 0xb, 0xc, 0xf, 8, 1, 6, 0xa, 0xd, 4, 3];

pub fn ms(state: &mut State) {
    let pre_perm = *state;
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI_M[idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[pi_row as usize][pi_col as usize] = pre_perm.0[row][col];
        }
    }
}

pub fn ms_inv(state: &mut State) {
    let pre_perm = *state;
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI_M_INV[idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[pi_row as usize][pi_col as usize] = pre_perm.0[row][col];
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dialga::{helper::state::State, ms::{ms, ms_inv}};

    #[test]
    fn test_ms() {
        let mut test_state = State::from(0x92a33e3c3115979441131a892119bed7);
        ms(&mut test_state);
        ms_inv(&mut test_state);
        assert_eq!(State::from(0x92a33e3c3115979441131a892119bed7), test_state);
    }
}