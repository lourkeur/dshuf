use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use sha2::{Digest, Sha256};

pub fn pick<T>(randomness: &[u8], mut input: Vec<T>, output_len: usize) -> Vec<T> {
    let input_len = input.len();
    for i in 0..output_len {
        let mut h = Sha256::new();
        h.update(randomness);
        h.update(BigUint::from(i).to_bytes_be());
        let r = BigUint::from_bytes_be(&h.finalize()[..]);
        input.swap(i, i + (r % (input_len - i)).to_usize().unwrap());
    }
    input.resize_with(output_len, || panic!("unreachable"));
    input
}
