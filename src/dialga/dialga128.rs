use crate::dialga::ms::{ms, ms_inv};
use crate::dialga::roundfunction::r_i::*;
use crate::dialga::helper::state::{self, State};
use crate::dialga::roundconstants::{*};
use crate::dialga::roundfunction::sub_cell::{sub_cell, sub_cell_inv};

const ALPHA:usize = 4;
const BETA:usize = ALPHA-1;

pub fn encrypt(plaintext: u128, tweak: u128, key: [u128; 2]) -> u128 {
    let mut state_d = State::from(plaintext ^ key[0] ^ key[1] ^ tweak);
    let mut state_t_rf = [State::from(0); ALPHA];
    let mut state_t_rm = [State::from(0); 2];
    let mut state_t_rb = [State::from(0); BETA];

    tweak_schedule(State::from(tweak), key, &mut state_t_rf, &mut state_t_rm, &mut state_t_rb);

    r_f(&mut state_d, &state_t_rf, key);
    r_m(&mut state_d, &state_t_rm, key);
    r_b(&mut state_d, &state_t_rb, key);
    let ciphertext: u128 = state_d.into();
    ciphertext 
}

pub fn decrypt(ciphertext: u128, tweak: u128, key: [u128; 2]) -> u128 {
    let mut state_d = State::from(ciphertext);
    let mut state_t_rf = [State::from(0); ALPHA];
    let mut state_t_rm = [State::from(0); 2];
    let mut state_t_rb = [State::from(0); BETA];

    tweak_schedule(State::from(tweak), key, &mut state_t_rf, &mut state_t_rm, &mut state_t_rb);

    r_b_inv(&mut state_d, &state_t_rb, key);
    r_m_inv(&mut state_d, &state_t_rm, key);
    r_f_inv(&mut state_d, &state_t_rf, key);

    let plaintext:u128 = state_d.into();
    plaintext ^ key[0] ^ key[1] ^ tweak
}

fn tweak_schedule(tweak: State, key: [u128; 2], state_t_rf: &mut [State; ALPHA], state_t_rm: &mut [State; 2], state_t_rb: &mut [State; BETA]) {
    //R_F schedule
    for i in 1..=ALPHA {
        if i == 1 {state_t_rf[i-1] = tweak ^ key[(i-1)%2];}
        else {
            let mut t_i = state_t_rf[i-2]; // previous state
            r_i(&mut t_i, (i-1)%4);
            state_t_rf[i-1] = t_i ^ key[(i-1)%2];
        }
    }
    let mut tmp_state = state_t_rf[ALPHA-1];

    //R_M Schedule
    state_t_rm[0] = sub_cell_inv(&mut tmp_state);
    sub_cell_inv(&mut tmp_state);
    tmp_state = tmp_state ^ key[(ALPHA -1)%2]; 
    state_t_rm[1] = r_i_inv(&mut tmp_state, (ALPHA-1)%4);

    //R_B Schedule
    for i in 1..=BETA {
        if i == BETA {
            state_t_rb[i-1] = state_t_rb[i-2] ^ key[(ALPHA-i-1)%2];
        }
        else {
            let mut t_i;
            if i == 1 {
                t_i = tmp_state;
            } else {
                t_i = state_t_rb[i-1];
            }
            t_i ^= key[(ALPHA-i-1)%2];
            r_i_inv(&mut t_i, (ALPHA-i-1)%4);
            state_t_rb[i-1] = t_i;
        }
    }
}

fn r_f(state_d: &mut State, t_r: &[State; ALPHA], key: [u128; 2]) {
    for i in 1..=ALPHA {
        r_i(state_d, (2*i-2)%4);
        *state_d ^= key[i%2] ^ C_F[2*(i-1)];

        let state_printout:u128 = State::into(*state_d);
        println!("{:x}", state_printout);

        r_i(state_d, (2*i-2)%4);
        *state_d ^= t_r[i-1] ^ C_F[2*(i-1)];

        let state_printout:u128 = State::into(*state_d);
        println!("{:x}", state_printout);
    }
}

fn r_m(state_d: &mut State, t_m: &[State; 2], key: [u128; 2]) {
    //Data Schedule
    r_i(state_d, (2*ALPHA)%4);
    *state_d ^=  t_m[0] ^ key[(ALPHA - 1)%2] ^ C_M[0];

    r_i(state_d, (2*ALPHA + 1)%4);
    let mut t_m_1 = t_m[1];
    *state_d ^= ms(&mut t_m_1) ^ C_M[1];
}

fn r_b(state_d: &mut State, t_b: &[State; BETA], key: [u128; 2]) {
    //Data Schedule
    for i in 1..=BETA {
        r_i(state_d, (2*(ALPHA+i))%4);
        *state_d ^= key[(ALPHA+i+1)%2] ^ C_B[2*(i-1)];

        r_i(state_d, (2*(ALPHA+i)+1)%4);
        let mut t_b_i = t_b[i-1];
        ms(&mut t_b_i);
        *state_d ^= t_b_i ^ C_B[2*i -1];
    }
    sub_cell_inv(state_d);
    *state_d ^= key[0] ^ key[1];
}

fn r_f_inv(state_d: &mut State, t_r: &[State; ALPHA], key: [u128; 2]) {
    for i in (1..=ALPHA).rev() {
        let state_printout:u128 = State::into(*state_d);
        println!("{:x}", state_printout);

        *state_d ^= t_r[i-1] ^ C_F[2*(i-1)];
        r_i_inv(state_d, (2*i-2)%4);

        let state_printout:u128 = State::into(*state_d);
        println!("{:x}", state_printout);
        
        *state_d ^= key[i%2] ^ C_F[2*(i-1)];
        r_i_inv(state_d, (2*i-2)%4);
    }
}

fn r_m_inv(state_d: &mut State, t_m: &[State; 2], key: [u128; 2]) {
    // State schedule
    let mut t_m_1 = t_m[1];
    *state_d ^= ms(&mut t_m_1) ^ C_M[1];
    r_i_inv(state_d, (2*ALPHA + 1)%4);
    *state_d ^=  t_m[0] ^ key[(ALPHA - 1)%2] ^ C_M[0];
    r_i_inv(state_d, (2*ALPHA)%4);
}

fn r_b_inv(state_d: &mut State, t_b: &[State; BETA], key: [u128; 2]) {
    *state_d ^= key[0] ^ key[1];
    sub_cell_inv(state_d);

    // Data Schedule
    for i in (1..=BETA).rev() {
        let mut t_b_i = t_b[i-1];
        ms(&mut t_b_i);
        *state_d ^= t_b_i ^ C_B[2*i -1];
        r_i_inv(state_d, (2*(ALPHA+i)+1)%4);
        

        *state_d ^= key[(ALPHA+i+1)%2] ^ C_B[2*(i-1)];
        r_i_inv(state_d, (2*(ALPHA+i))%4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PAINTEXT:u128 = 0x00112233445566778899aabbccddeeff;
    const KEY: [u128; 2] = [
        0x00112233445566778899aabbccddeeff,
        0x112233445566778899aabbccddeeff00,
    ];
    const TWEAK:u128 = 0x2233445566778899aabbccddee00ff11;

    fn prepare_tests() -> ([state::State; ALPHA], [state::State; 2], [state::State; BETA]) {
        let mut state_t_rf = [State::from(0); ALPHA];
        let mut state_t_rm = [State::from(0); 2];
        let mut state_t_rb = [State::from(0); BETA];

        tweak_schedule(State::from(TWEAK), KEY, &mut state_t_rf, &mut state_t_rm, &mut state_t_rb);
        assert_eq!(State::from(TWEAK), state_t_rb[BETA-1]);

        return (state_t_rf, state_t_rm, state_t_rb);
    }

    

    #[test]
    fn test_encryption_roundabout() { // works for now, but reverse schedule for r_b needs to be implemented, currently xor with tweak does not work!!
        let ciphertext = encrypt(PAINTEXT, TWEAK, KEY);
        let decrypted = decrypt(ciphertext, TWEAK, KEY);
        assert_eq!(PAINTEXT, decrypted);
    }

    #[test]
    fn test_encryption_test_vector() {
        let ciphertext = encrypt(PAINTEXT, TWEAK, KEY);
        assert_eq!(0x838407143af9a876fbdc6be378e9045b, ciphertext);
    }

    #[test]
    fn test_encryption_rf() {
        let (state_d_rf, _, _) = prepare_tests();
        let mut test_case = State::from(PAINTEXT);
        r_f(&mut test_case, &state_d_rf, KEY);
        r_f_inv(&mut test_case, &state_d_rf, KEY);
        assert_eq!(State::from(PAINTEXT), test_case);
    }

    #[test]
    fn test_encryption_rm() { // only for r_m --> tested for correctness already
        let (_, state_d_rm, _) = prepare_tests();
        let mut test_case = State::from(PAINTEXT);
        r_m(&mut test_case, &state_d_rm, KEY);
        r_m_inv(&mut test_case, &state_d_rm, KEY);
        assert_eq!(State::from(PAINTEXT), test_case);
    }

    #[test]
    fn test_encryption_rb() {
        let (_, _, state_d_rb) = prepare_tests();
        let mut test_case = State::from(PAINTEXT);
        r_b(&mut test_case, &state_d_rb, KEY);
        r_b_inv(&mut test_case, &state_d_rb, KEY);
        assert_eq!(State::from(PAINTEXT), test_case);
    }


}
