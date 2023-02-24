use drand_verify::{derive_randomness, g1_from_fixed, verify};

use hex;

fn main() {
    let mut PK_LEO_MAINNET = [0u8; 48];
    hex::decode_to_slice("868f005eb8e6e4ca0a47c8a77ceaa5309a47978a7c71bc5cce96366b5d7a569937c529eeda66c7293784a9402801af31", &mut PK_LEO_MAINNET).unwrap();
    // TODO: no hardcoding
    let round_number = 1337;
    let api_baseurl = "https://drand.cloudflare.com";
    let chainHash = "8990e7a9aaed2ffed73dbd7092123d6f289930540d7651336225dc172e51b2ce";
    let pubkey = g1_from_fixed(PK_LEO_MAINNET).unwrap();
    let signature = hex::decode("80d95247ddf1bb3acf5738497a5f10406be283144603f63d714bb1a44ff6b93285ae2697fffeb50c68862bd9fbecd4b204b1798d2686b4ac5d573615031d9d67e6168bde9a7adf1161430a498ca701a25c216aee3e38ffd5290369034fa050a2").unwrap();

    // TODO
    //dbg!(surf::get(format!("{api_baseurl}/public/{round_number}")));
    verify(&pubkey, round_number, &[], &signature).unwrap();
    println!("{}", hex::encode(derive_randomness(&signature)));
}
