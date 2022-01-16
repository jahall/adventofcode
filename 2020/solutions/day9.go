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
	//part2()
}

func part1(xmas *Xmas) {
	index, num := xmas.FindBreak()
	fmt.Printf("PART 1: First number is %d at index %d\n", num, index)
}

func part2() {
	return
}

func atoi(str string) (int, error) {
	val, err := strconv.Atoi(str)
	return val, err
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
