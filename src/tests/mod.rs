use test_case::test_case;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use std::fs::File;
use std::io::BufReader;

use crate::shuffle;

#[derive(Serialize, Deserialize)]
struct TestCase {
    input: Vec<String>,
    randomness: String,
    limit: usize,
    output: Vec<String>,
}

impl TestCase {
    fn parse(name: &str) -> Self {
        let file = File::open(format!("src/tests/testcases/{name}.json")).unwrap();
         let reader = BufReader::new(file);
        from_reader(reader).unwrap()
    }
}

#[test_case("basic")]
#[test_case("basic_less")]
#[test_case("basic_more")]
#[test_case("basic_other_input")]
#[test_case("basic_other_randomness")]
fn it_works(name: &str) {
    let tc = TestCase::parse(name);
    let input: Vec<Vec<u8>> = tc.input.iter().map(|v| hex::decode(v).unwrap()).collect();
    let input: Vec<&[u8]> = input.iter().map(|v| v.as_slice()).collect();
    let randomness = hex::decode(tc.randomness).unwrap();
    let limit = tc.limit;
    let output: Vec<Vec<u8>> = tc.output.iter().map(|v| hex::decode(v).unwrap()).collect();
    let output: Vec<&[u8]> = output.iter().map(|v| v.as_slice()).collect();
    assert_eq!(output, shuffle(randomness[..].try_into().unwrap(), input, limit));
}
