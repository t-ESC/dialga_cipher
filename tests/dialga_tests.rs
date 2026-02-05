

#[cfg(test)]
mod tests {
    use dialga_cipher_rust::dialga::ms::*;
    use dialga_cipher_rust::dialga::roundconstants;
    use dialga_cipher_rust::dialga::roundfunction::*;
    use dialga_cipher_rust::dialga::helper::bitarray::*;
    use dialga_cipher_rust::dialga::helper::state::*;

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
    fn test_permute_bits_and_inv() {
        for i in 0..4 {
            println!("Testing i={}", i);
            for test_case in 0..=0xFF {
                let permuted = permute_bits(test_case, i);
                let result = permute_bits_inv(permuted, i);
                assert_eq!(test_case, result);
            }
        }
    }

    #[test]
    fn test_sub_cell_old() {
        for i in 0..4 {
            let mut state = TEST_STATE;
            sub_cell_old(&mut state, i);
            sub_cell_old(&mut state, i);
            assert_eq!(TEST_STATE, state);

        }
    }

    #[test]
    fn test_sub_cell() {
        let mut state = TEST_STATE;
        sub_cell(&mut state);
        sub_cell(&mut state);
        assert_eq!(TEST_STATE, state);
    }

    #[test]
    fn test_matrix_mul_old() {
        let mut state: State = TEST_STATE;

        for i in 0..4 {
            matrix_mul(&mut state, i);
            matrix_mul(&mut state, i);
            assert_eq!(TEST_STATE, state);
        }
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
        r_i(&mut test_state, 0);
        let output:u128 = test_state.into();
        println!("{:b}", output);
        println!("{:b}", desired_output);
        assert_eq!(output, desired_output);
    }

    #[test]
    fn test_roundfunction_1() {
        let r_1:u128 = 0x9f95f3ff7a092a2c465dfdf31225ea00;
        let r_2:u128 = 0x98871f6b568e38c69b2df2b8fecc46c4;

        let desired_output = r_2 ^ KEY[0] ^ TWEAK ^ roundconstants::C_F[1];
        let mut output_state = State::from(r_1);
        r_i(&mut output_state, 1);
        let output:u128 = output_state.into();
        assert_eq!(output, desired_output);
    }

    #[test]
    fn test_sub_cell_outside_r_i() {
        let desired_output = CIPHERTEXT ^ KEY[0] ^ KEY[1];
        let r16: u128 = 0x92a33e3c3115979441131a892119bed7;
        let mut test_state = State::from(r16);

        sub_cell(&mut test_state);
        let result:u128 = test_state.into();

        assert_eq!(desired_output, result);
    }

}

