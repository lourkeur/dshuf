use drand_verify::{derive_randomness, g1_from_fixed, verify};

use futures::executor::block_on;

use hex;
use std::io::{self, Read};

use dshuf::drand_api;
use dshuf::shuffle;

fn main() {
    let mut PK_LEO_MAINNET = [0u8; 48];
    hex::decode_to_slice("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31", &mut PK_LEO_MAINNET).unwrap();
    // TODO: no hardcoding
    let round_number = 1337;
    let pubkey = g1_from_fixed(PK_LEO_MAINNET).unwrap();

    // TODO
    let (signature, previous_signature) =
        block_on(drand_api::get_signatures(round_number)).unwrap();

    verify(&pubkey, round_number, &previous_signature, &signature).unwrap();
    let randomness = derive_randomness(&signature);

    // simulate shuf -n 3
    let mut stdin = io::stdin();
    let mut buf = Vec::new();
    stdin.read_to_end(&mut buf).unwrap();
    let separator = '\n';
    let mut input = Vec::from_iter(buf.split(|c| *c == separator as u8));
    if input.last().map_or(false, |e| e.len() == 0) {
        input.truncate(input.len()-1);
    }
    let output = shuffle(&randomness, input, 3);
    println!("{:?}", output);
}
