#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct State(pub [[u8; 4];4]);

impl State {
    pub fn new(data: [[u8; 4];4]) {
        State(data);
    }

    pub fn from_flat(data: [u8; 16]) -> State {
        let mut mat = [[0_u8; 4];4];
        for i in 0..16 {
            let j = i % 4;
            let k = i / 4;
            mat[j][k] = data[i]; 
        }
        State(mat)
    }
}

impl From<[u8; 16]> for State {
    fn from(value: [u8; 16]) -> Self {
        State::from_flat(value)
    }
}

impl From<u128> for State {
    fn from(value: u128) -> Self {
        State::from_flat(value.to_be_bytes())
    }
}