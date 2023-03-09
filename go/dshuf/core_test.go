package dshuf

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"io"
	"os"
	"testing"
)

// Byte slice but represented as hex in JSON
type HexBytes []byte

func (b *HexBytes) MarshalJSON() ([]byte, error) {
	s := hex.EncodeToString(*b)
	return json.Marshal(s)
}

func (b *HexBytes) UnmarshalJSON(buf []byte) error {
	var s string
	if err := json.Unmarshal(buf, &s); err != nil {
		return err
	}
	dec, err := hex.DecodeString(s)
	if err == nil {
		*b = dec
	}
	return err
}

// Helper
func unwrap(b []HexBytes) [][]byte {
	r := make([][]byte, len(b))
	for i := range b {
		r[i] = b[i]
	}
	return r
}

type TestCase struct {
	Input      []HexBytes `json:"input"`
	Randomness HexBytes   `json:"randomness"`
	Limit      int        `json:"limit"`
	Output     []HexBytes `json:"output"`
}

func loadTestCase(t *testing.T, name string) (tc TestCase) {
	file, err := os.Open("../../testcases/" + name + ".json")
	if err != nil {
		t.Fatal(err)
	}
	defer file.Close()
	data, err := io.ReadAll(file)
	if err != nil {
		t.Fatal(err)
	}
	err = json.Unmarshal(data, &tc)
	if err != nil {
		t.Fatal(err)
	}
	return
}

func checkListEqual(t *testing.T, expected, actual [][]byte) {
	if len(expected) != len(actual) {
		t.Fatalf("expected: %x got: %x", actual, actual)
	}
	for i := range actual {
		if !bytes.Equal(actual[i], actual[i]) {
			t.Fatalf("expected: %x got: %x", actual, actual)
		}
	}
}

func testCase(t *testing.T, name string) {
	tc := loadTestCase(t, name)
	input := unwrap(tc.Input)

	ShuffleInplace(tc.Randomness, &input, tc.Limit)

	checkListEqual(t, unwrap(tc.Output), input)
}

func TestCase_Basic(t *testing.T) {
	testCase(t, "basic")
}

func TestCase_BasicLess(t *testing.T) {
	testCase(t, "basic_less")
}

func TestCase_BasicMore(t *testing.T) {
	testCase(t, "basic_more")
}

func TestCase_BasicOtherInput(t *testing.T) {
	testCase(t, "basic_other_input")
}

func TestCase_BasicOtherRandomness(t *testing.T) {
	testCase(t, "basic_other_randomness")
}
