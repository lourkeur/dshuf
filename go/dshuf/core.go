package dshuf

import (
	"github.com/zeebo/blake3"

	"encoding/binary"
	"math/big"
)

const SAMPLE_LEN = 24

func ShuffleInplace(
	randomness []byte,
	input *[][]byte,
	limit int,
) {
	es := *input
	if limit > len(es) {
		limit = len(es)
	}
	h, _ := blake3.NewKeyed(randomness)
	for _, e := range es {
		var buf [8]byte
		binary.BigEndian.PutUint64(buf[:], uint64(len(e)))
		h.Write(buf[:])
		h.Write(e)
	}

	prng := h.Digest()
	for i := 0; i < limit; i++ {
		var sample [SAMPLE_LEN]byte
		prng.Read(sample[:])
		r := new(big.Int).SetBytes(sample[:])
		r.Mod(r, big.NewInt(int64(len(es)-i)))
		j := i + int(r.Uint64())
		es[i], es[j] = es[j], es[i]
	}
	*input = es[:limit]
}
