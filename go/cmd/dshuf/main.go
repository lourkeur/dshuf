package main

import (
	"fmt"
	"github.com/lourkeur/dshuf/go/dshuf"
	"log"
)

func main() {
	const rn = 1337 // TODO: no hardcoding
	rep, err := dshuf.GetDrandSignatures(rn)
	if err != nil {
		log.Fatal(err)
	}
	randomness := rep.Randomness()

	// simulate shuf -n 3
	entries := [][]byte{[]byte("Alice"), []byte("Bob"), []byte("Carla"), []byte("David")}
	dshuf.ShuffleInplace(randomness, &entries, 3)
	fmt.Println(entries)
}
