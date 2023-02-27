use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;

const SAMPLE_LEN: usize = 24;

pub fn pick<T>(randomness: &[u8; 32], mut input: Vec<T>, output_len: usize) -> Vec<T> {
    let input_len = input.len();
    let mut samples = vec![0u8;SAMPLE_LEN*output_len];
    let h = blake3::Hasher::new_keyed(randomness);
    // TODO: seed prng with shuffle inputs
    h.finalize_xof().fill(samples.as_mut_slice());
    for (i, sample) in samples.chunks(SAMPLE_LEN).enumerate() {
        let r = BigUint::from_bytes_be(sample);
        input.swap(i, i + (r % (input_len - i)).to_usize().unwrap());
    }
    input.resize_with(output_len, || panic!("unreachable"));
    input
}
