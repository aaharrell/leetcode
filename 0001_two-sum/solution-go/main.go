package main

import (
	"fmt"
	"solution-go/providers"
)


func main() {
	fmt.Println("Running program...")
	fmt.Println(twoSum.TwoSum([]int{2, 7, 11, 15}, 9))
	fmt.Println(twoSum.TwoSum([]int{3, 2, 4}, 6))
	fmt.Println(twoSum.TwoSum([]int{3, 3}, 6))
}
