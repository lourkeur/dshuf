use drand_verify::{derive_randomness, g1_from_fixed, verify};

use futures::executor::block_on;

use hex;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use sha2::{Digest, Sha256};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct DrandResponse {
    round: u64,
    randomness: String,
    signature: String,
    previous_signature: String,
}

fn main() {
    let mut PK_LEO_MAINNET = [0u8; 48];
    hex::decode_to_slice("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31", &mut PK_LEO_MAINNET).unwrap();
    // TODO: no hardcoding
    let round_number = 1337;
    let api_baseurl = "https://drand.cloudflare.com";
    let pubkey = g1_from_fixed(PK_LEO_MAINNET).unwrap();

    // TODO
    let rep = block_on(surf::get(format!("{api_baseurl}/public/{round_number}")).recv_json::<DrandResponse>()).unwrap();
    dbg!(&rep);

    let signature = hex::decode(rep.signature).unwrap();
    let previous_signature = hex::decode(rep.previous_signature).unwrap();

    verify(&pubkey, round_number, &previous_signature, &signature).unwrap();
    let randomness = derive_randomness(&signature);
    println!("{}", hex::encode(randomness));

    // simulate shuf -n 3
    let mut input = vec!("Alice", "Bob", "Carla", "David");
    let input_len = input.len();
    let output_len = 3;
    for i in 0..output_len {
        let mut h = Sha256::new();
        h.update(randomness);
        h.update(BigUint::from(i).to_bytes_be());
        let r = BigUint::from_bytes_be(&h.finalize()[..]);
        input.swap(i, i+ (r % (input_len - i)).to_usize().unwrap());
    }
    input.resize_with(output_len, || panic!("unreachable"));
    let output = input;
    println!("{:?}", output);
}
