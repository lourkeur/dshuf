use crate::shuffle;

#[test]
fn it_works() {
    let input = vec![&b"Alice"[..], &b"Bob"[..], &b"Carla"[..], &b"David"[..]];
    let randomness = [38, 96, 102, 79, 141, 75, 196, 1, 25, 77, 128, 216, 29, 162, 10, 30, 121, 72, 15, 101, 184, 226, 210, 5, 174, 203, 209, 67, 181, 191, 176, 211];
    let actual = shuffle(&randomness, input, 3);
    assert_eq!(actual, vec![&b"Bob"[..], &b"Carla"[..], &b"Alice"[..]])
}
