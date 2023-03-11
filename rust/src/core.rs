use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

use core::cmp::min;

const SAMPLE_LEN: usize = 24;

pub fn shuffle<'a>(
    randomness: &[u8; 32],
    mut input: Vec<&'a [u8]>,
    limit: usize,
) -> Vec<&'a [u8]> {
    let input_len = input.len();
    let output_len = min(limit, input_len);
    let mut h = blake3::Hasher::new_keyed(randomness);
    for e in &input {
        h.update(&(e.len() as u64).to_be_bytes());
        h.update(e);
    }
    let mut prng = h.finalize_xof();
    for i in 0..output_len {
        let mut sample = [0u8; SAMPLE_LEN];
        prng.fill(&mut sample);
        let r = BigUint::from_bytes_be(&sample);
        input.swap(i, i + (r % (input_len - i)).to_usize().unwrap());
    }
    input.truncate(output_len);
    input
}
