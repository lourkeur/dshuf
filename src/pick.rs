use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

const SAMPLE_LEN: usize = 24;

pub fn pick<'a>(
    randomness: &[u8; 32],
    mut input: Vec<&'a [u8]>,
    output_len: usize,
) -> Vec<&'a [u8]> {
    let input_len = input.len();
    let mut h = blake3::Hasher::new_keyed(randomness);
    for e in &input {
        h.update(e);
        h.update(&[0u8]);
    }
    let mut prng = h.finalize_xof();
    for i in 0..input_len {
        let mut sample = [0u8; SAMPLE_LEN];
        prng.fill(&mut sample);
        let r = BigUint::from_bytes_be(&sample);
        input.swap(i, i + (r % (input_len - i)).to_usize().unwrap());
    }
    input.resize_with(output_len, || panic!("unreachable"));
    input
}
