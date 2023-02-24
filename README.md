# generate reproducible random permutations using public randomness

`dshuf` aims to mimic the interface of the Unix `shuf` command
while getting its entropy from a [League Of Entropy] public randomness beacon.
This can be used for simple lotteries for example:
operators commit to a round number in the future and reveal a list of participants,
and the canonical winners can be reproducibly computed once the round occurs.

[League Of Entropy]: https://drand.love/

## Roadmap (non-committal)
- [ ] MVP
- [ ] Add `-n` command line option
- [ ] Add Go implementation
- [ ] Add Web implementation
