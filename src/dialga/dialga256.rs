use crate::dialga::ms::{ms};
use crate::dialga::roundfunction::r_i::*;
use crate::dialga::helper::state::{State};
use crate::dialga::roundconstants::{*};
use crate::dialga::roundfunction::sub_cell::{sub_cell};

const ALPHA:usize = 5;
const BETA:usize = ALPHA-1;

pub fn encrypt(plaintext: u128, tweak: [u128; 2], key: [u128; 2]) -> u128 {
    let mut state_d = State::from(plaintext ^ key[0] ^ key[1] ^ tweak[0]);
    let mut state_t0_rf = [State::from(0); ALPHA];
    let mut state_t0_rm = [State::from(0); 2];
    let mut state_t0_rb = [State::from(0); BETA];
    let mut state_t1_rf = [State::from(0); ALPHA-1];
    let mut state_t1_rm = State::from(0);
    let mut state_t1_rb = [State::from(0); BETA+1];

    tweak_schedule0(State::from(tweak[0]), key, &mut state_t0_rf, &mut state_t0_rm, &mut state_t0_rb);
    tweak_schedule1(State::from(tweak[1]), key, &mut state_t1_rf, &mut state_t1_rm, &mut state_t1_rb);

    r_f(&mut state_d, &state_t0_rf, &State::from(tweak[1]), &state_t1_rf, key);
    r_m(&mut state_d, &state_t0_rm, &state_t1_rm);
    r_b(&mut state_d, &state_t0_rb, &state_t1_rb, key);

    let ciphertext: u128 = state_d.into();
    ciphertext 
}

pub fn decrypt(ciphertext: u128, tweak: [u128; 2], key: [u128; 2]) -> u128 {
    let mut state_d = State::from(ciphertext);
    let mut state_t0_rf = [State::from(0); ALPHA];
    let mut state_t0_rm = [State::from(0); 2];
    let mut state_t0_rb = [State::from(0); BETA];
    let mut state_t1_rf = [State::from(0); ALPHA-1];
    let mut state_t1_rm = State::from(0);
    let mut state_t1_rb = [State::from(0); BETA+1];

    tweak_schedule0(State::from(tweak[0]), key, &mut state_t0_rf, &mut state_t0_rm, &mut state_t0_rb);
    tweak_schedule1(State::from(tweak[1]), key, &mut state_t1_rf, &mut state_t1_rm, &mut state_t1_rb);

    r_b_inv(&mut state_d, &state_t0_rb, &state_t1_rb, key);
    r_m_inv(&mut state_d, &state_t0_rm, &state_t1_rm);
    r_f_inv(&mut state_d, &state_t0_rf, &State::from(tweak[1]), &state_t1_rf, key);
    
    let plaintext:u128 = state_d.into();
    plaintext ^ key[0] ^ key[1] ^ tweak[0]
}

fn tweak_schedule0(
    tweak0: State, 
    key: [u128; 2], 
    state_t_rf: &mut [State; ALPHA], 
    state_t_rm: &mut [State; 2], 
    state_t_rb: &mut [State; BETA]
) {
    for i in 1..=ALPHA { //R_F schedule
        if i == 1 {
            state_t_rf[i-1] = tweak0 ^ key[(i-1)%2]
        } else {
            let mut t_i = state_t_rf[i-2];
            r_i(&mut t_i, (i-1)%4);
            state_t_rf[i-1] = t_i ^ key[(i-1)%2];
        }
    }

    // R_M Schedule
    let mut tmp_state = state_t_rf[ALPHA-1];
    sub_cell(&mut tmp_state);
    state_t_rm[0] = tmp_state;
    sub_cell(&mut tmp_state);
    tmp_state ^= key[(ALPHA-1)%2];
    r_i_inv(&mut tmp_state, (ALPHA-1)%4);
    state_t_rm[1] = tmp_state;

    //R_B Schedule
    for i in 1..=BETA{
        if i == BETA {
            state_t_rb[i-1] = state_t_rb[i-2] ^ key[(ALPHA-i-1)%2];
        }
        else {
            let mut t_i;
            if i == 1 {
                t_i = tmp_state;
            } else {
                t_i = state_t_rb[i-2];
            }
            t_i ^= key[(ALPHA-i-1)%2];
            r_i_inv(&mut t_i, (ALPHA-i-1)%4);
            state_t_rb[i-1] = t_i;
        }
    }
}

fn tweak_schedule1(
    tweak1: State, 
    key: [u128; 2], 
    state_t_rf: &mut [State; ALPHA-1], 
    state_t_rm: &mut State, 
    state_t_rb: &mut [State; BETA+1]
) {
    // R_F schedule
    for i in 1..=ALPHA {
        if i == 1 {
            state_t_rf[i-1] = tweak1 ^ key[i%2]
        } else {
            let mut t_i = state_t_rf[i-2];
            r_i(&mut t_i, (i-1)%4);
            if i == ALPHA {
                *state_t_rm = t_i ^ key[i%2];
            } else {
                state_t_rf[i-1] = t_i ^ key[i%2];
            }
        }
    }

    // R_M Schedule
    let mut tmp_state = *state_t_rm;
    tmp_state ^= key[ALPHA%2];
    r_i_inv(&mut tmp_state, (ALPHA-1)%4);
    state_t_rb[0] = tmp_state;


    // R_B Schedule
    for i in 1..=BETA {
        if i == BETA {
            state_t_rb[i] = state_t_rb[i-1] ^ key[(ALPHA-i)%2]; 
        } else {
            let mut t_i = state_t_rb[i-1];
            t_i ^= key[(ALPHA-i)%2];
            r_i_inv(&mut t_i, (ALPHA-i-1)%4);
            state_t_rb[i] = t_i;
        }
    }
}

fn r_f(state_d: &mut State, t0_r: &[State; ALPHA], t1_0: &State, t1_r: &[State; ALPHA-1], key: [u128; 2]) {
    for i in 1..=ALPHA{
        
        r_i(state_d, (2*i-2)%4);
        if i == 1 {
            *state_d ^= *t1_0 ^ key[i%2] ^ C_F[2*(i-1)];
        } else {
            *state_d ^= t1_r[i-2] ^ C_F[2*(i-1)];
        }        

        r_i(state_d, (2*i-1)%4);
        *state_d ^= t0_r[i-1] ^ C_F[2*i-1];        
    }
}

fn r_f_inv(state_d: &mut State, t0_r: &[State; ALPHA], t1_0: &State, t1_r: &[State; ALPHA-1], key: [u128; 2]) {
    for i in (1..=ALPHA).rev() {
        *state_d ^= t0_r[i-1] ^ C_F[2*i-1];
        r_i_inv(state_d, (2*i-1)%4);

        if i == 1 {
            *state_d ^= *t1_0 ^ key[i%2] ^ C_F[2*(i-1)];
        } else {
            *state_d ^= t1_r[i-2] ^ C_F[2*(i-1)];
        }
        r_i_inv(state_d, (2*i-2)%4);

    }
}

fn r_m(state_d: &mut State, t0_m: &[State; 2], t1_m: &State) {
    r_i(state_d, (2*ALPHA)%4);
    *state_d ^= t0_m[0] ^ *t1_m ^ C_M[0];

    r_i(state_d, (2*ALPHA + 1)%4);
    let mut t0_m_1 = t0_m[1];
    ms(&mut t0_m_1);
    *state_d ^= t0_m_1 ^ C_M[1];
}

fn r_m_inv(state_d: &mut State, t0_m: &[State; 2], t1_m: &State) {
    let mut t0_m_1 = t0_m[1];
    ms(&mut t0_m_1);
    *state_d ^= t0_m_1 ^ C_M[1];
    r_i_inv(state_d, (2*ALPHA + 1)%4);

    *state_d ^= t0_m[0] ^ *t1_m ^ C_M[0];
    r_i_inv(state_d, (2*ALPHA)%4);
}

fn r_b(state_d: &mut State, t0_b: &[State; BETA], t1_b: &[State; BETA+1],key: [u128; 2]) {

    for i in 1..=BETA {
        r_i(state_d, (2*(ALPHA+1))%4);
        let mut t1_b_i = t1_b[i-1];
        ms(&mut t1_b_i);
        *state_d ^= t1_b_i ^ C_B[2*(i-1)]; 


        r_i(state_d, (2*(ALPHA+i)+1)%4);
        let mut t0_b_i = t0_b[i-1];
        ms(&mut t0_b_i);
        *state_d ^= t0_b_i ^ C_B[2*i-1];
    }

    sub_cell(state_d);
    let mut t1_b_beta = t1_b[BETA];
    ms(&mut t1_b_beta);
    *state_d ^= t1_b_beta ^ key[0] ^ key[1];
}

fn r_b_inv(state_d: &mut State, t0_b: &[State; BETA], t1_b: &[State; BETA+1],key: [u128; 2]) {

    let mut t1_b_beta = t1_b[BETA];
    ms(&mut t1_b_beta);
    *state_d ^= t1_b_beta ^ key[0] ^ key[1];
    sub_cell(state_d);
    
    for i in (1..=BETA).rev() {
        let mut t0_b_i = t0_b[i-1];
        ms(&mut t0_b_i);
        *state_d ^= t0_b_i ^ C_B[2*i-1]; 
        r_i_inv(state_d, (2*(ALPHA+i)+1)%4);

        let mut t1_b_i = t1_b[i-1];
        ms(&mut t1_b_i);
        *state_d ^= t1_b_i ^ C_B[2*(i-1)]; 
        r_i_inv(state_d, (2*(ALPHA+1))%4);
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
    const TWEAK: [u128; 2] = [
        0x2233445566778899aabbccddeeff1122,
        0x33445566778899aabbccddeeff001100,
    ];

    fn prepare_tests() -> ([State; ALPHA], [State; ALPHA-1], [State; 2], State, [State; BETA], [State; BETA+1]) {
        let mut state_t0_rf = [State::from(0); ALPHA];
        let mut state_t0_rm = [State::from(0); 2];
        let mut state_t0_rb = [State::from(0); BETA];
        let mut state_t1_rf = [State::from(0); ALPHA-1];
        let mut state_t1_rm = State::from(0);
        let mut state_t1_rb = [State::from(0); BETA+1];

        tweak_schedule0(State::from(TWEAK[0]), KEY, &mut state_t0_rf, &mut state_t0_rm, &mut state_t0_rb);
        tweak_schedule1(State::from(TWEAK[1]), KEY, &mut state_t1_rf, &mut state_t1_rm, &mut state_t1_rb);

        assert_eq!(State::from(TWEAK[0]), state_t0_rb[BETA-1]);
        assert_eq!(State::from(TWEAK[1]), state_t1_rb[BETA]);

        // println!("{:x}", TWEAK[1]);
        // for tweak in state_t1_rf {
        //     let state:u128 = State::into(tweak);
        //     println!("{:x}", state);   
        // }

        return(state_t0_rf, state_t1_rf, state_t0_rm, state_t1_rm, state_t0_rb, state_t1_rb);


    }

    #[test]
    fn test_encryption_roundabout() {
        let ciphertext = encrypt(PAINTEXT, TWEAK, KEY);
        let decrypted = decrypt(ciphertext, TWEAK, KEY);
        assert_eq!(PAINTEXT, decrypted);
    }

    #[test]
    fn test_encryption_rf() {
        let (state_t0_rf, state_t1_rf, _, _, _, _) = prepare_tests();
        let mut test_case = State::from(PAINTEXT ^ KEY[0] ^ KEY[1] ^ TWEAK[0]);
        r_f(&mut test_case, &state_t0_rf, &State::from(TWEAK[1]), &state_t1_rf, KEY);
        let test_case_u128:u128 = State::into(test_case);
        assert_eq!(0x70d4ed7e8620b096b86f7f2ada600dc1, test_case_u128);
        r_f_inv(&mut test_case, &state_t0_rf, &State::from(TWEAK[1]), &state_t1_rf, KEY);
        assert_eq!(State::from(PAINTEXT ^ KEY[0] ^ KEY[1] ^ TWEAK[0]), test_case);
    }

    #[test]
    fn test_encryption_rm() {
        let (_, _, state_t0_rm, state_t1_rm, _, _) = prepare_tests();
        let mut test_case = State::from(0x70d4ed7e8620b096b86f7f2ada600dc1);
        r_m(&mut test_case, &state_t0_rm, &state_t1_rm);
        let test_case_u128:u128 = State::into(test_case);
        assert_eq!(0x2e823a29ffd14704b33bb433c3ec52c0, test_case_u128);
        r_m_inv(&mut test_case, &state_t0_rm, &state_t1_rm);
        assert_eq!(State::from(0x70d4ed7e8620b096b86f7f2ada600dc1), test_case);

    }

    #[test]
    fn test_encryption_rb() {
        let (_, _, _, _, state_t0_rb, state_t1_rb) = prepare_tests();
        let mut test_case = State::from(0x2e823a29ffd14704b33bb433c3ec52c0);
        r_b(&mut test_case, &state_t0_rb, &state_t1_rb, KEY);
        let test_case_u128:u128 = State::into(test_case);
        assert_eq!(0xe129ff0920371753db65532540d06881, test_case_u128);
        r_b_inv(&mut test_case, &state_t0_rb, &state_t1_rb, KEY);
        assert_eq!(State::from(0x2e823a29ffd14704b33bb433c3ec52c0), test_case);       
    }


}