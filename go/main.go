package main

import (
	"fmt"
)

func slice_append(s *[]int) {
	*s = append(*s, 1)
}

func main() {
	var s []int
	slice_append(&s)
	fmt.Println(s)
}
