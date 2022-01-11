package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// Policy struct used to check passwords
type Policy struct {
	char  string
	lower int
	upper int
}

func (p *Policy) isValidByBounds(password string) bool {
	occurrences := strings.Count(password, p.char)
	return (occurrences >= p.lower) && (occurrences <= p.upper)
}

func (p *Policy) isValidByLocation(password string) bool {
	match1 := password[p.lower-1] == p.char[0]
	match2 := password[p.upper-1] == p.char[0]
	return (match1 || match2) && !(match1 && match2)
}

// Useful for passing out of loadData
type PolicyPasswordPair struct {
	policy   Policy
	password string
}

func main() {
	data := loadData()
	part1(data)
	part2(data)
}

func part1(data []PolicyPasswordPair) {
	nValid := 0
	for _, pair := range data {
		if pair.policy.isValidByBounds(pair.password) {
			nValid++
		}
	}
	fmt.Printf("PART 1: %d valid passwords\n", nValid)
}

func part2(data []PolicyPasswordPair) {
	nValid := 0
	for _, pair := range data {
		if pair.policy.isValidByLocation(pair.password) {
			nValid++
		}
	}
	fmt.Printf("PART 2: %d valid passwords\n", nValid)
}

func loadData() []PolicyPasswordPair {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day2.txt")
	check(err)
	scanner := bufio.NewScanner((file))
	var pairs []PolicyPasswordPair
	re := regexp.MustCompile("[- ]")
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), ": ")
		parts := re.Split(line[0], 3)
		policy := Policy{char: parts[2], lower: atoi(parts[0]), upper: atoi(parts[1])}
		pair := PolicyPasswordPair{policy: policy, password: line[1]}
		pairs = append(pairs, pair)
	}
	return pairs
}

// Utility to simply ignore errors on conversion
func atoi(str string) int {
	val, _ := strconv.Atoi(str)
	return val
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
