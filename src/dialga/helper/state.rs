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