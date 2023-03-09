use serde::{Deserialize, Serialize};

use hex;

#[derive(Debug, Serialize, Deserialize)]
struct DrandResponse {
    round: u64,
    randomness: String,
    signature: String,
    previous_signature: String,
}

const API_BASEURL: &str = "https://drand.cloudflare.com";
pub async fn get_signatures(round_number: u64) -> surf::Result<(Vec<u8>, Vec<u8>)> {
    let rep = surf::get(format!("{API_BASEURL}/public/{round_number}"))
        .recv_json::<DrandResponse>()
        .await?;
    let signature = hex::decode(rep.signature).unwrap();
    let previous_signature = hex::decode(rep.previous_signature).unwrap();
    Ok((signature, previous_signature))
}
