use std::ops::{BitXor, BitXorAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct State(pub [[u8; 4];4]);

impl State {
    pub fn new(data: [[u8; 4];4]) -> State {
        State(data)
    }

    pub fn from_flat(data: [u8;16]) -> State {
        let mut mat = [[0_u8; 4];4];
        for i in 0..16 {
            let row = i & 0b0011;
            let col = i >> 2;
            mat[row][col] = data[i];
        }
        State(mat)
    }
}

impl From<[u8; 16]> for State {
    fn from(value: [u8; 16]) -> Self {
        State::from_flat(value)
    }
}

impl Into<[u8; 16]> for State {
    fn into(self) -> [u8; 16] {
        let mut result = [0_u8; 16];
        for row in 0..4 {
            for col in 0..4 {
                let index = (col << 2) + row;
                result[index] = self.0[row][col];
            }
        }
        result 
    }
}

impl From<u128> for State {
    fn from(value: u128) -> Self {
        State::from_flat(value.to_be_bytes())
    }
}

impl Into<u128> for State {
    fn into(self) -> u128 {
        u128::from_be_bytes(self.into())
    }
}

impl BitXor for State {
    type Output = State;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result: State = State::from(0_u128);
        for row in 0..4 {
            for col in 0..4 {
                result.0[row][col] = self.0[row][col] ^ rhs.0[row][col];
            }
        }
        result
    }
}

impl BitXorAssign for State {
    fn bitxor_assign(&mut self, rhs: Self) {
        for row in 0..4 {
            for col in 0..4 {
                self.0[row][col] ^= rhs.0[row][col];
            }
        }
    }
}

impl BitXor<u128> for State {
    type Output = State;

    fn bitxor(self, rhs: u128) -> Self::Output {
        let mut result = State::from(rhs);
        for row in 0..4 {
            for col in 0..4 {
                result.0[row][col] ^= self.0[row][col];
            }
        }
        result
    }
}

impl BitXorAssign<u128> for State {
    fn bitxor_assign(&mut self, rhs: u128) {
        let rhs_as_state = State::from(rhs);
        *self ^= rhs_as_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_STATE: State = State([
            [0xFF, 0x01, 0x02, 0x03],
            [0x10, 0x11, 0x12, 0x13],
            [0x20, 0x21, 0x22, 0x23],
            [0x30, 0x31, 0x32, 0x33],
        ]);

    const TEST_NUMBER: u128 = 0xFF102030011121310212223203132333_u128;
    
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

}