use crate::dialga::helper::state::State;

#[deprecated]
pub fn matrix_mul_old(state: &mut State, i: usize) { // make state mutalble for now (i think this is the better way, can check AES impl later
    let pre_mix:State = *state;

    // Wrong!!
    state.0[0][i] = pre_mix.0[1][i] ^ pre_mix.0[2][i] ^ pre_mix.0[3][i];
    state.0[1][i] = pre_mix.0[0][i] ^ pre_mix.0[2][i] ^ pre_mix.0[3][i];
    state.0[2][i] = pre_mix.0[0][i] ^ pre_mix.0[1][i] ^ pre_mix.0[3][i];
    state.0[3][i] = pre_mix.0[0][i] ^ pre_mix.0[1][i] ^ pre_mix.0[2][i];
}

pub fn matrix_mul(state: &mut State) -> State { //Midori shuffles every column of the matrix, maybe they do here too

    /* i-th State column multiplied with Matrix --> self inverse
    * (0 1 1 1)
    * (1 0 1 1)
    * (1 1 0 1)
    * (1 1 1 0)*/

    let pre_mix: [[u8; 4]; 4] = state.0;

    for col in 0..4 {
        state.0[0][col] = pre_mix[1][col] ^ pre_mix[2][col] ^ pre_mix[3][col];
        state.0[1][col] = pre_mix[0][col] ^ pre_mix[2][col] ^ pre_mix[3][col];
        state.0[2][col] = pre_mix[0][col] ^ pre_mix[1][col] ^ pre_mix[3][col];
        state.0[3][col] = pre_mix[0][col] ^ pre_mix[1][col] ^ pre_mix[2][col];
    }

    *state
}