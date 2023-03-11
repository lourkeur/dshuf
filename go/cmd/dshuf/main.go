package main

import (
	"bytes"
	"github.com/lourkeur/dshuf/go/dshuf"
	"io"
	"log"
	"os"
)

func main() {
	const rn = 1337 // TODO: no hardcoding
	rep, err := dshuf.GetDrandSignatures(rn)
	if err != nil {
		log.Fatal(err)
	}
	randomness := rep.Randomness()

	// simulate shuf -n 3
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}
	separator := []byte("\n")
	entries := bytes.Split(input, separator)
	if len(entries[len(entries)-1]) == 0 {
		entries = entries[:len(entries)-1]
	}
	dshuf.ShuffleInplace(randomness, &entries, 3)
	for _, e := range entries {
		os.Stdout.Write(e)
		os.Stdout.Write(separator)
	}
}
