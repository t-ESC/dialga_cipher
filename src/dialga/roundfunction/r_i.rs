use crate::dialga::helper::state::State;
use crate::dialga::roundfunction::matrix_mul::*;
use crate::dialga::roundfunction::sub_cell::*;
use crate::dialga::roundfunction::permutation::*;

pub fn r_i(state: &mut State, i: usize) -> State { // roundfunction, called r_1 in paper
    sub_cell_inv(state);
    byte_permutation(state, i);
    matrix_mul(state);
    *state
}

pub fn r_i_inv(state: &mut State, i: usize) -> State {
    matrix_mul(state);
    byte_permutation_inv(state, i); //only asymmetric function
    sub_cell_inv(state);
    *state
}

#[cfg(test)]
mod tests {
    use crate::dialga::{helper::state::*, roundfunction::{r_i::r_i}};
    #[test]
    fn test_vector_for_r_0() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x2fac1e416eb12146303ba35f33f8e75d, 0x586f3bfb96a3e48aa00a7884a034f5d2];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            r_i(&mut test_state, 0);
            assert_eq!(State::from(test_vectors[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_r_1() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x1eac412f5d33e7f8b1216e46305f3ba3, 0x3b6ffb58d2a0f534a3e4968aa0840a78];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            r_i(&mut test_state, 1);
            assert_eq!(State::from(test_vectors[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_r_2() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x2f1e41ac33e75df86e2146b130a35f3b, 0x583bfb6fa0f5d23496e48aa3a078840a];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            r_i(&mut test_state, 2);
            assert_eq!(State::from(test_vectors[i]), test_state);
        }
    }

    #[test]
    fn test_vector_for_r_3() {
        let testcases: [u128; _] = [0x9f95f3ff7a092a2c465dfdf31225ea00, 0x98871f6b568e38c69b2df2b8fecc46c4];
        let test_vectors: [u128; _] = [0x704cf07e8af208ae526c37cdd3d78dc3, 0x9e83410e347bd78285116c3b7dce3445];

        for (i, testcase) in testcases.iter().enumerate() {
            let mut test_state = State::from(*testcase);
            r_i(&mut test_state, 3);
            assert_eq!(State::from(test_vectors[i]), test_state);
        }
    }
}

