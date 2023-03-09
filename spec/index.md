# The dshuf Protocol

DRAFT 2023-03-02

The dshuf protocol allows any number of participants
that don't trust each other
to generate a random permutation of an arbitrary list
using a random beacon.


## Limitations

No entry in the input list contains the null byte and the number of entries must not exceed `2⁶⁴ - 1`.

Negligible probability is defined as less than or equal to `2⁻¹²⁸`.

The random beacon is modeled as a trusted third party that broadcasts an unpredictable 256-bit value at the agreed upon time.
It is trusted not to reveal any part of this value in advance, or collude with participants to bias it.
To implement this functionality, the dshuf tool employs the [LoE DRand](https://drand.love) random beacon.
Thus, users must assume that DRand is working as designed when they use dshuf.


## Procedures
### Set up

Participants must agree on a canonical representation of the items they want to shuffle,
which must consist of a byte representation of each entry and an initial order.
They must also agree on when and how the random beacon will be consulted.
In the DRand case they must choose a round number in the future.


### Shuffling

All participants consult the random beacon to learn the random value.
They then execute the following program:
- evaluate the `blake3` extendable output function with the random value as the key,
and the null-separated canonical representation of the entries as input.
- Store the input list in an array `t` in canonical order.
- Then, for `i` in `0 .. len(t)`:
  - pull the next 192 bits from the output of `blake3` and interpret them as a big-endian integer value `r`.
  - swap `t[i]` and `t[i + (r mod (len(t)-i))]`.
- Return the final contents of `t` as the shuffled output.


## Security Goals
### Agreement

Given the same input list and round number,
no two honest participants derive different output lists.


### Fairness

All permutations of the input lists are equally likely to be selected.
(except with negligible probability)


### Robustness

Arbitrary alterations of the input list
by a computationally bounded adversary
occurring strictly before the random beacon reveal
do not result in bias in the output.
(except with negligible probability)


### Absence of cross-instance interaction

When two different lists are shuffled,
the outputs are independent.
(except with negligible probability)
