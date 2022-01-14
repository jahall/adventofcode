package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// Struct for group
type Group struct {
	answers []string
}

func MakeGroup() Group {
	var answers []string
	return Group{answers: answers}
}

func (s *Group) AnyYesCount() int {
	return len(s.yesCounts())
}

func (s *Group) AllYesCount() int {
	allYes := 0
	for _, count := range s.yesCounts() {
		if count == len(s.answers) {
			allYes += 1
		}
	}
	return allYes
}

func (s *Group) yesCounts() map[rune]int {
	counts := make(map[rune]int)
	for _, answers := range s.answers {
		for _, q := range answers {
			counts[q] += 1
		}
	}
	return counts
}

func main() {
	groups := loadGroups()
	part1(groups)
	part2(groups)
}

func part1(groups []Group) {
	count := 0
	for _, group := range groups {
		count += group.AnyYesCount()
	}
	fmt.Printf("PART 1: Total any yes questions is %d\n", count)
}

func part2(groups []Group) {
	count := 0
	for _, group := range groups {
		count += group.AllYesCount()
	}
	fmt.Printf("PART 2: Total all yes questions is %d\n", count)
}

func loadGroups() []Group {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day6.txt")
	check(err)
	var groups []Group
	scanner := bufio.NewScanner((file))
	var group = MakeGroup()
	for scanner.Scan() {
		answers := strings.TrimSpace(scanner.Text())
		if answers == "" {
			groups = append(groups, group)
			group = MakeGroup()
			continue
		}
		group.answers = append(group.answers, answers)
	}
	groups = append(groups, group)
	return groups
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
