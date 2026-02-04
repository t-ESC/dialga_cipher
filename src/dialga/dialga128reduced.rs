use crate::dialga::roundfunction::*;
use crate::dialga::helper::state::{State};
use crate::dialga::roundconstants::*;

const ALPHA:usize = 4;

pub fn encrypt(plaintext: u128, tweak: u128, key: [u128; 2]) -> u128 {
    let mut state_d = State::from(plaintext ^ key[0] ^ key[1] ^ tweak);
    let state_t = State::from(tweak);

    r_f(&mut state_d, &state_t, key);
    // r_m
    // r_b
    let ciphertext: u128 = state_d.into();
    ciphertext 
}

pub fn decrypt(ciphertext: u128, tweak: u128, key: [u128; 2]) -> u128 {
    println!("{:x}", ciphertext);
    let mut state_d = State::from(ciphertext);
    let state_t = State::from(tweak);

    r_f_inv(&mut state_d, &state_t, key);

    let plaintext:u128 = state_d.into();
    plaintext ^ key[0] ^ key[1] ^ tweak
}

fn r_f(state_d: &mut State, tweak: &State, key: [u128; 2]) {
    // Tweak Schedule
    let mut t_r: [State; ALPHA] = [State::from(0); ALPHA]; // Index will be -1 from round since round starts at 1 (whyyyyyy)
    for i in 1..=ALPHA {
        if i == 1 {t_r[i-1] = *tweak ^ key[(i-1)%2];}
        else {
            let mut t_i = t_r[i-2]; // previous state
            r_i(&mut t_i, (i-1)%4);
            t_r[i-1] = t_i ^ key[(i-1)%2];
        }
    }

    for i in 1..=ALPHA {
        if i == 1 {
            // Round 1
            r_i(state_d, 0);
            *state_d ^= key[1] ^ C_F[0];
            // Round 2
            r_i(state_d, 1);
            *state_d ^= t_r[0] ^ C_F[1];
        } else {
            // round 2i-2
            r_i(state_d, (2*i-2)%4);
            *state_d ^= key[i%2] ^ C_F[2*i - 2];
            // round 2i-1
            r_i(state_d, (2*i-1)%4);
            *state_d ^= t_r[i-1] ^ C_F[2*i - 1];
        }
    }
}

fn r_f_inv(state_d: &mut State, tweak: &State, key: [u128; 2]) {
    // Tweak Schedule
    let mut t_r: [State; ALPHA] = [State::from(0); ALPHA]; // Index will be -1 from round since round starts at 1 (whyyyyyy)
    for i in 1..=ALPHA {
        if i == 1 {t_r[i-1] = *tweak ^ key[(i-1)%2];}
        else {
            let mut t_i = t_r[i-2]; // previous state
            r_i(&mut t_i, (i-1)%4);
            t_r[i-1] = t_i ^ key[(i-1)%2];
        }
    }

    for i in (1..=ALPHA).rev() {
        if i == 1 {
            *state_d ^= t_r[0] ^ C_F[1];
            r_i_inv(state_d, 1);
            *state_d ^= key[1] ^ C_F[0];
            r_i_inv(state_d, 0);
        } else {
            *state_d ^= t_r[i-1] ^ C_F[2*i - 1];
            r_i_inv(state_d, (2*i-1)%4);

            *state_d ^= key[i%2] ^ C_F[2*i - 2];
            r_i_inv(state_d, (2*i-2)%4);
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::dialga::dialga128reduced::*;
    const PAINTEXT:u128 = 0x00112233445566778899aabbccddeeff;
    const KEY: [u128; 2] = [
        0x00112233445566778899aabbccddeeff,
        0x112233445566778899aabbccddeeff00,
    ];
    const TWEAK:u128 = 0x2233445566778899aabbccddeeff0011;

    #[test]
    fn test_encryption_goaround() {
        let ciphertext = encrypt(PAINTEXT, TWEAK, KEY);
        let plaintext = decrypt(ciphertext, TWEAK, KEY);
        assert_eq!(PAINTEXT, plaintext);
    }

    #[test]
    fn r_f_goaround() {
        let mut plaintext = State::from(PAINTEXT);
        let mut tweak = State::from(TWEAK);

        r_f(&mut plaintext, &mut tweak, KEY);
        r_f_inv(&mut plaintext, &mut tweak, KEY);

        assert_eq!(State::from(PAINTEXT), plaintext);
    }
    
}
