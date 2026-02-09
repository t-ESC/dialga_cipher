

#[cfg(test)]
mod tests {
    use dialga_cipher_rust::dialga::helper::bitarray::*;
    use dialga_cipher_rust::dialga::helper::state::*;
    
    use dialga_cipher_rust::dialga::ms::*;
    use dialga_cipher_rust::dialga::roundconstants;
    use dialga_cipher_rust::dialga::roundfunction::r_i::*;
    
    use dialga_cipher_rust::dialga::roundfunction::matrix_mul::*;
    use dialga_cipher_rust::dialga::roundfunction::sub_cell::*;
    use dialga_cipher_rust::dialga::roundfunction::permutation::*;

    const TEST_STATE: State = State([
            [0xFF, 0x01, 0x02, 0x03],
            [0x10, 0x11, 0x12, 0x13],
            [0x20, 0x21, 0x22, 0x23],
            [0x30, 0x31, 0x32, 0x33],
        ]);

    const TEST_NUMBER: u128 = 0xFF102030011121310212223203132333_u128;

    const PAINTEXT:u128 = 0x00112233445566778899aabbccddeeff;
    const KEY: [u128; 2] = [
        0x00112233445566778899aabbccddeeff,
        0x112233445566778899aabbccddeeff00,
    ];
    const TWEAK:u128 = 0x2233445566778899aabbccddeeff0011;
    const CIPHERTEXT:u128 = 0x838407143af9a876fbdc6be378e9045b; // of dialga 128 reduced

    #[test]
    fn test_state_from() {
        let state1 = State::from([0xFF_u8, 0x10, 0x20, 0x30, 0x01, 0x11, 0x21, 0x31, 0x02, 0x12, 0x22, 0x32, 0x03, 0x13, 0x23, 0x33]);
        assert_eq!(TEST_STATE, state1);
        let state2 = State::from(0xFF102030011121310212223203132333_u128);
        assert_eq!(TEST_STATE, state2);
    }

    #[test]
    fn test_state_into() {
        let num1:u128 = State::into(TEST_STATE);
        assert_eq!(TEST_NUMBER, num1);
    }

    #[test]
    fn test_state_xor() {
        let state1 = State::from(0xFF102030011121310212223203132333_u128);
        let mut case = State::from(0_u128);
        case ^= state1;
        assert_eq!(TEST_STATE, case);
    }

    #[test]
    fn test_bitarray_from_into() {
        for test_case in 0..=0xFF_u8 {
            let result_array = BitArray::from(test_case);
            let result:u8 = result_array.into();

            assert_eq!(test_case, result);
        }
    }
    #[test]
    fn test_permute_bits_specific_value() {
        let testcase = 0xF_u8;
        let result = permute_bits(testcase, 1);

        assert_eq!(195, result);

        let resconstructed = permute_bits_inv(result, 1);
        assert_eq!(0xf, resconstructed);
    }



    #[test]
    fn test_permute_bits_and_inv() {
        for i in 0..4 {
            for test_case in 0..=0xFF {
                let permuted = permute_bits(test_case, i);
                let result = permute_bits_inv(permuted, i);
                assert_eq!(test_case, result);
            }
        }
    }

    #[test]
    fn test_sub_cell() {
        let mut state = State::from(PAINTEXT);
        sub_cell_inv(&mut state);
        sub_cell_inv(&mut state);
        assert_eq!(State::from(PAINTEXT), state);
    }

    #[test]
    fn test_matrix_mul_old() {
        let mut state: State = TEST_STATE;

        for i in 0..4 {
            matrix_mul_old(&mut state, i);
            matrix_mul_old(&mut state, i);
            assert_eq!(TEST_STATE, state);
        }
    }
    #[test]
    fn test_matrix_mul() {
        let mut state: State = TEST_STATE;

        matrix_mul(&mut state);
        matrix_mul(&mut state);
        assert_eq!(TEST_STATE, state);
    }

    #[test]
    fn test_byte_perm() {
        let mut state = TEST_STATE;

        for i in 0..4 {
            byte_permutation(&mut state, i);
            byte_permutation_inv(&mut state, i);
            assert_eq!(TEST_STATE, state)
        }
    }

    #[test]
    fn roundfunction_roundabout() {
        let mut state = TEST_STATE;
        for i in 0..4 {
            r_i(&mut state, i);
            r_i_inv(&mut state, i);
            assert_eq!(TEST_STATE, state);
        }
    }

    #[test]
    fn test_ms() {
        for test_case in 0..=0xF {
            assert_eq!(test_case, PI_M_INV[PI_M[test_case as usize] as usize])
        }
    }

    #[test]
    fn test_ms_impl() {
        let mut test_case = TEST_STATE;
        ms(&mut test_case);
        ms_inv(&mut test_case);
        assert_eq!(TEST_STATE, test_case)
    }

    #[test]
    fn test_roundfunction_0(){
        let start = PAINTEXT ^ TWEAK ^ KEY[0] ^ KEY[1];
        let r_1:u128 = 0x9f95f3ff7a092a2c465dfdf31225ea00;
        let desired_output  = r_1 ^ KEY[1] ^ roundconstants::C_F[0];
        
        let mut test_state = State::from(start);
        println!("{:?}", test_state);
        r_i(&mut test_state, 0);
        assert_eq!(State::from(desired_output), test_state);
    }

    #[test]
    fn test_roundfunction_1() {
        let r_1:u128 = 0x9f95f3ff7a092a2c465dfdf31225ea00;
        let r_2:u128 = 0x98871f6b568e38c69b2df2b8fecc46c4;

        let desired_output = r_2 ^ KEY[0] ^ TWEAK ^ roundconstants::C_F[1];
        let mut output_state = State::from(r_1);
        r_i(&mut output_state, 1);
        assert_eq!(State::from(desired_output), output_state);
    }

    #[test]
    fn test_sub_cell_outside_r_i() {
        let output = CIPHERTEXT ^ KEY[0] ^ KEY[1];
        let r16: u128 = 0x92a33e3c3115979441131a892119bed7;
        let mut test_state = State::from(r16);

        println!("{:?}", test_state);

        sub_cell_inv(&mut test_state);

        assert_eq!(State::from(output), test_state);
    }

    #[test]
    fn test_sub_cell_outside_r_i_reverse() {
        let r16: u128 = 0x92a33e3c3115979441131a892119bed7;
        let output: u128 = CIPHERTEXT ^ KEY[0] ^ KEY[1];
        let mut test_state = State::from(output);

        println!("{:?}", test_state);

        sub_cell_inv(&mut test_state);
        assert_eq!(State::from(r16), test_state); // Works only if substitution 1 and 3 are flipped
        // Also need to replace PI_2 with entirely different permutations (which then make it match what we expect)
    }

    #[test]
    fn test_sub_cell_outside_r_i_dialga256() { // MS might be incorrect
        let mut tweak_1 = State::from(0x33445566778899aabbccddeeff001100);
        ms(&mut tweak_1);
        let ms_tweak_1:u128 = tweak_1.into();
        let desired_output:u128 = CIPHERTEXT ^ KEY[0] ^ KEY[1] ^ ms_tweak_1;
        let r16:u128 = 0x4daa1d40e36de6bdda58801d83a4aa6b;

        let mut test_state = State::from(r16);
        sub_cell_inv(&mut test_state);
        assert_eq!(State::from(desired_output), test_state);

    }


    #[test]
    fn test_sub_cell_single_row_reverse() {
        let permuted_state = State([[0; 4], [0; 4], [22, 185, 122, 21], [0; 4]]);
        let first_state = State([[0; 4], [0; 4], [62, 151, 26, 190], [0; 4]]);

        let mut test_state = permuted_state;
        sub_cell_inv(&mut test_state);

        assert_eq!(first_state.0[2], test_state.0[2]);

    }

}

