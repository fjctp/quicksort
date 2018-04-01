package main

import (
	"fmt"
)

type f64 []float64

func (slice f64) sort() []float64 {
	sorted := make(f64, len(slice))

	return sorted
}

func main() {
	unsort := f64{5, 10, 1, 2, 5, 3, 4}
	sorted := unsort.sort()

	fmt.Println(unsort)
	fmt.Println(sorted)
}