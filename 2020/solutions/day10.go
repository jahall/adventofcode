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

func LoadAdapterBag(test bool) *AdapterBag {
	suffix := ""
	if test {
		suffix = "_test"
	}
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day10" + suffix + ".txt")
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

func (a *AdapterBag) CalcNumArrangements() int {
	numArrangements := 1
	prevRating := 0
	subStart := 0
	maxRating := a.ratings[len(a.ratings)-1]
	padded := append([]int{0}, a.ratings...)
	padded = append(padded, maxRating+3)
	for i, rating := range padded {
		if rating-prevRating == 3 {
			subRatings := padded[subStart:i]
			arrs := a.findArrangements(subRatings)
			// Below useful for testing
			//fmt.Println()
			//for _, arr := range arrs {
			//	fmt.Println(arr)
			//}
			numArrangements *= len(arrs)
			subStart = i
		}
		prevRating = rating
	}
	return numArrangements
}

func (a *AdapterBag) findArrangements(ratings []int) [][]int {
	if len(ratings) <= 2 {
		return [][]int{ratings}
	}
	arrangements := [][]int{}
	// Drop nothing
	for _, tail := range a.findArrangements(ratings[1:]) {
		arr := a.concat(ratings[:1], tail)
		arrangements = append(arrangements, arr)
	}
	// Drop second rating
	if ratings[2]-ratings[0] <= 3 {
		for _, tail := range a.findArrangements(ratings[2:]) {
			arr := a.concat(ratings[:1], tail)
			arrangements = append(arrangements, arr)
		}
	}
	// Drop second and third ratings
	if len(ratings) > 3 && ratings[3]-ratings[1] <= 3 {
		for _, tail := range a.findArrangements(ratings[3:]) {
			arr := a.concat(ratings[:2], tail)
			arrangements = append(arrangements, arr)
		}
	}
	return arrangements
}

func (a *AdapterBag) concat(arr1 []int, arr2 []int) []int {
	// Append actually alters the array in-place! Hence copying
	arr := make([]int, len(arr1))
	copy(arr, arr1)
	return append(arr, arr2...)
}

func part1(bag *AdapterBag) {
	diffs := bag.CalcJoltDiffs()
	fmt.Printf("PART 1: 1-jolt (%d) x 3-jolt (%d) = %d\n", diffs[1], diffs[3], diffs[1]*diffs[3])
}

func part2(bag *AdapterBag) {
	arrangements := bag.CalcNumArrangements()
	fmt.Printf("PART 2: There are %d distinct ways of connecting\n", arrangements)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	bag := LoadAdapterBag(test)
	part1(bag)
	part2(bag)
}
