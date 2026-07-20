pub mod dialga128;
pub mod dialga256;
pub mod helper;
pub mod roundfunction;
pub mod ms;
pub mod roundconstants;

#[cfg(test)]
mod tests {
    use crate::dialga::{helper::state::State, roundfunction::r_i::{r_i, r_i_inv}};
    use super::*;
    use std::thread;
    use rand::RngExt;
    use indicatif::{ProgressBar, ProgressStyle};

    fn experimental_verification(
        input_delta: u128, 
        output_delta: u128, 
        n_rounds: usize,
        first_round: usize,
        backwards: bool,
        n_trials: usize,
        n_threads: usize
    ) -> u64 {
        let chunk = (n_trials + n_threads -1) / n_threads;
        let mut handles: Vec<std::thread::JoinHandle<u64>> = Vec::with_capacity(n_threads);

        for t in 0..n_threads {
            let start = t*chunk;
            let end = (start + chunk).min(n_trials);

            handles.push(thread::spawn(move || {
                let mut local_count:u64 = 0;
                let mut rng = rand::rng();

                for _ in start..end {
                    let first_plaintext: u128 = rng.random();
                    let second_plaintext: u128 = first_plaintext ^ input_delta; // Calculated input differences

                    let mut first_state: State = State::from(first_plaintext);
                    let mut second_state: State = State::from(second_plaintext);

                    for i in 0..n_rounds {
                        if !backwards {
                            r_i(&mut first_state, (first_round + i) % 4);
                            r_i(&mut second_state, (first_round + i) % 4);
                        } else {
                            r_i_inv(&mut first_state, 3-(first_round + i) % 4);
                            r_i_inv(&mut second_state, 3-(first_round + i) % 4);
                        }
                    }

                    let first_result:u128 = first_state.into();
                    let second_result:u128 = second_state.into();

                    if first_result ^ second_result == output_delta {
                        local_count += 1;
                    }
                }
                local_count
            }));
        }

        let total:u64 = handles.into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .sum();

        return total;
    }

    #[test]
    fn binomial_test() {
        let input_delta: u128 = 0x00000000000000000000000001000000;
        let output_delta: u128 = 0x01100400000100040000044004000004;

        let n_rounds:usize = 2;
        let first_round:usize = 0;
        let backwards:bool = true;

        let n_trials:usize = 335_544_320;
        let n_threads:usize = 12;

        let result = experimental_verification(
            input_delta,
            output_delta,
            n_rounds,
            first_round,
            backwards,
            n_trials,
            n_threads
        );

        println!("{}", result);
    }
}