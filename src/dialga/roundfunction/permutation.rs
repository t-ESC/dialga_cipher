use crate::dialga::helper::state::*;

const PI: [[usize; 16];4] = [
	[7, 0, 13, 10, 5, 2, 15, 8, 4, 3, 14, 9, 6, 1, 12, 11],
	[13, 0, 10, 7, 11, 6, 12, 1, 2, 15, 5, 8, 4, 9, 3, 14],
	[7, 13, 10, 0, 6, 12, 11, 1, 5, 15, 8, 2, 4, 14, 9, 3],
	[13, 8, 6, 3, 14, 11, 5, 0, 12, 9, 7, 2, 15, 10, 4, 1],
];

const PI_INV:[[usize; 16];4] = [
	[1, 13, 5, 9, 8, 4, 12, 0, 7, 11, 3, 15, 14, 2, 10, 6],
	[1, 7, 8, 14, 12, 10, 5, 3, 11, 13, 2, 4, 6, 0, 15, 9],
	[3, 7, 11, 15, 12, 8, 4, 0, 10, 14, 2, 6, 5, 1, 13, 9],
	[7, 15, 11, 3, 14, 6, 2, 10, 1, 9, 13, 5, 8, 0, 4, 12],
];

pub fn byte_permutation(state: &mut State, r: usize) -> State {
    let pre_perm = *state;
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI[r][idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[row][col] = pre_perm.0[pi_row][pi_col];
        }
    }
    *state
}

pub fn byte_permutation_inv(state: &mut State, r: usize) -> State {
    let pre_perm = *state;
    for row in 0..4 {
        for col in 0..4 {
            let idx = (col << 2) + row;
            let pi_idx = PI_INV[r][idx];

            let pi_col = pi_idx >> 2;
            let pi_row = pi_idx & 0b0011;

            state.0[row][col] = pre_perm.0[pi_row][pi_col];
        }
    }
    *state
}

mod tests {
    use super::*;
    #[test]
    fn test_vector_for_permutation_0() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x2c9f25fd09f300467affea5d2a9512f3, 0xc698ccf28e1fc49b566b462d3887feb8];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            byte_permutation(&mut test_state, 0);
            assert_eq!(State::from(test_vectors[i]), test_state);
            byte_permutation_inv(&mut test_state, 0);
            assert_eq!(State::from(testcases[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_permutation_1() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x259ffd2cf32a1295f30009467a5dffea, 0xcc98f2c6b838fe871fc48e9b562d6b46];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            byte_permutation(&mut test_state, 1);
            assert_eq!(State::from(test_vectors[i]), test_state);
            byte_permutation_inv(&mut test_state, 1);
            assert_eq!(State::from(testcases[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_permutation_2() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x2c25fd9f2a12f395090046f37aea5dff, 0xc6ccf29838feb8878ec49b1f56462d6b];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            byte_permutation(&mut test_state, 2);
            assert_eq!(State::from(test_vectors[i]), test_state);
            byte_permutation_inv(&mut test_state, 2);
            assert_eq!(State::from(testcases[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_permutation_3() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x25462affeaf3099f125d2cf300fd7a95, 0xcc9b386b46b88e98fe2dc61fc4f25687];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            byte_permutation(&mut test_state, 3);
            assert_eq!(State::from(test_vectors[i]), test_state);
            byte_permutation_inv(&mut test_state, 3);
            assert_eq!(State::from(testcases[i]), test_state);
        }
    }
}