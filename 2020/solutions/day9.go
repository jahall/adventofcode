package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Struct for the Xmas cypher
type Xmas struct {
	numbers []int
	window  []int
	cache   [][]int
	index   int
}

func LoadXmas(preamble int) *Xmas {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day9.txt")
	check(err)
	var numbers []int
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		num, _ := strconv.Atoi(line)
		numbers = append(numbers, num)
	}
	return NewXmas(numbers, preamble)
}

func NewXmas(numbers []int, preamble int) *Xmas {
	// Not-very-concise way of defining 2d array
	// https://stackoverflow.com/questions/39804861/what-is-a-concise-way-to-create-a-2d-slice-in-go
	cache := make([][]int, preamble)
	for i := range cache {
		cache[i] = make([]int, preamble)
	}
	for i := 0; i < preamble; i++ {
		for j := i + 1; j < preamble; j++ {
			cache[i][j] = numbers[i] + numbers[j]
		}
	}
	xmas := Xmas{
		numbers: numbers,
		window:  numbers[:preamble],
		cache:   cache,
		index:   preamble,
	}
	return &xmas
}

// Find the encryption weakness
func (x *Xmas) FindWeakness() int {
	_, invalidNum := x.FindBreak()
	for i := 0; i < len(x.numbers); i++ {
		agg, offset := 0, 0
		for {
			agg += x.numbers[i+offset]
			if agg == invalidNum {
				min, max := x.findMinMax(x.numbers[i : i+offset+1])
				return min + max
			}
			if agg > invalidNum {
				break
			}
			offset++
		}
	}
	return -1
}

func (x *Xmas) findMinMax(nums []int) (int, int) {
	// Interestingly no builtins for finding min / max of slice
	// https://stackoverflow.com/questions/34259800/is-there-a-built-in-min-function-for-a-slice-of-int-arguments-or-a-variable-numb
	min, max := -1, -1
	for _, val := range nums {
		if min == -1 || val < min {
			min = val
		}
		if max == -1 || val > max {
			max = val
		}
	}
	return min, max
}

// Find the breaking number (and its index)
func (x *Xmas) FindBreak() (int, int) {
	for {
		index, num, valid := x.valid()
		if !valid {
			return index, num
		}
		x.advance()
	}
}

// Is the next number valid?
func (x *Xmas) valid() (int, int, bool) {
	number := x.numbers[x.index]
	for i := 0; i < len(x.window); i++ {
		for j := i + 1; j < len(x.window); j++ {
			if x.cache[i][j] == number {
				return x.index, number, true
			}
		}
	}
	return x.index, number, false
}

// Advance to next number and update cache
func (x *Xmas) advance() {
	windowIndex := x.index % len(x.window)
	x.window[windowIndex] = x.numbers[x.index]
	for i := 0; i < windowIndex; i++ {
		x.cache[i][windowIndex] = x.window[i] + x.window[windowIndex]
	}
	for j := windowIndex + 1; j < len(x.window); j++ {
		x.cache[windowIndex][j] = x.window[windowIndex] + x.window[j]
	}
	x.index++
}

func main() {
	xmas := LoadXmas(25)
	part1(xmas)
	part2(xmas)
}

func part1(xmas *Xmas) {
	index, num := xmas.FindBreak()
	fmt.Printf("PART 1: First number is %d at index %d\n", num, index)
}

func part2(xmas *Xmas) {
	num := xmas.FindWeakness()
	fmt.Printf("PART 2: Encryption weakness is %d\n", num)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
