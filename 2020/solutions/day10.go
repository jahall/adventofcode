package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

// Struct for your bag of adapters
type AdapterBag struct {
	ratings []int // assumed to be sorted
}

func LoadAdapterBag() *AdapterBag {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day10.txt")
	check(err)
	var ratings []int
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		rating, _ := strconv.Atoi(line)
		ratings = append(ratings, rating)
	}
	return NewAdapterBag(ratings)
}

func NewAdapterBag(ratings []int) *AdapterBag {
	sort.Ints(ratings)
	return &AdapterBag{ratings: ratings}
}

func (a *AdapterBag) CalcJoltDiffs() map[int]int {
	joltDiffs := make(map[int]int, 3)
	prev := 0
	for _, rating := range a.ratings {
		diff := rating - prev
		if diff < 1 || diff > 3 {
			fmt.Println("WARNING: Bad diff")
		}
		joltDiffs[diff] += 1
		prev = rating
	}
	joltDiffs[3] += 1 // since device rated for 3+ the max
	return joltDiffs
}

func main() {
	bag := LoadAdapterBag()
	part1(bag)
	part2(bag)
}

func part1(bag *AdapterBag) {
	diffs := bag.CalcJoltDiffs()
	fmt.Printf("PART 1: 1-jolt (%d) x 3-jolt (%d) = %d\n", diffs[1], diffs[3], diffs[1]*diffs[3])
}

func part2(bag *AdapterBag) {
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
