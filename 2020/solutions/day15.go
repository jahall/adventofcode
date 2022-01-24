package main

import (
	"fmt"
	"os"
)

func Simulate(numbers []int, end int) int {
	cache := make(map[int]int)
	var num int
	var lastNum int
	var lastNumSpokenBefore bool
	for turn := 0; turn < end; turn++ {
		// 1. Update turn number
		if turn < len(numbers) {
			num = numbers[turn]
		} else if lastNumSpokenBefore {
			num = turn - cache[lastNum] - 1
		} else {
			num = 0
		}
		// 2. Update cache for last number
		cache[lastNum] = turn - 1
		// 3. Update last number to this number
		lastNum = num
		_, lastNumSpokenBefore = cache[lastNum]
	}
	return num
}

func part1(numbers []int) {
	num := Simulate(numbers, 2020)
	fmt.Printf("PART 1: 2020th number spoken is %d\n", num)
}

func part2(numbers []int) {
	num := Simulate(numbers, 30000000)
	fmt.Printf("PART 1: 30,000,000th number spoken is %d\n", num)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	var numbers []int
	if test {
		numbers = []int{0, 3, 6}
	} else {
		numbers = []int{14, 3, 1, 0, 9, 5}
	}
	part1(numbers)
	part2(numbers)
}
