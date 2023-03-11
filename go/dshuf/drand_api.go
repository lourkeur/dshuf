package dshuf

import (
	"context"
	"github.com/drand/drand/client"
	"github.com/drand/drand/client/http"
)

const URL = "https://drand.cloudflare.com"

func GetDrandSignatures(roundNumber uint64) (client.Result, error) {
	ctx := context.Background() // we are a client
	client, err := http.New(URL, nil, nil)
	if err != nil {
		return nil, err
	}
	return client.Get(ctx, roundNumber)
}
