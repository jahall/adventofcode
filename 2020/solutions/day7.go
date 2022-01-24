package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strconv"
	"strings"
)

// Container for a bag, which can hold other bags
type Bag struct {
	color    string
	Counts   []int
	Children []*Bag
}

func NewBag(color string) *Bag {
	var children []*Bag
	return &Bag{color: color, Children: children}
}

// Does this bag contain a bag of this color somewhere down the tree?
func (b *Bag) Contains(color string) bool {
	for _, child := range b.Children {
		if child.color == color || child.Contains(color) {
			return true
		}
	}
	return false
}

// Calculate total bags required to be contained within this bag
func (b *Bag) TotalRequiredBags() int {
	count := 0
	for i, child := range b.Children {
		count += b.Counts[i]
		count += b.Counts[i] * child.TotalRequiredBags()
	}
	return count
}

func main() {
	bags := loadBags()
	part1(bags)
	part2(bags["shiny gold"])
}

func part1(bags map[string]*Bag) {
	count := 0
	for _, bag := range bags {
		if bag.Contains("shiny gold") {
			count++
		}
	}
	fmt.Printf("PART 1: There are %d bags which could contain shiny gold\n", count)
}

func part2(bag *Bag) {
	count := bag.TotalRequiredBags()
	fmt.Printf("PART 2: Shiny gold bag must contain %d bags\n", count)
}

func loadBags() map[string]*Bag {
	bags := make(map[string]*Bag)

	getOrMakeBag := func(color string) *Bag {
		if bags[color] == nil {
			bags[color] = NewBag(color)
		}
		return bags[color]
	}

	path, _ := filepath.Abs(filepath.Join("data", "day7.txt"))
	file, err := os.Open(path)
	check(err)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		regulation := strings.TrimSpace(scanner.Text())
		parseRegulation(regulation, getOrMakeBag)
	}
	return bags
}

func parseRegulation(regulation string, getOrMakeBag func(string) *Bag) {
	re := regexp.MustCompile(" bags contain ")
	parts := re.Split(regulation, 2)
	bag := getOrMakeBag(parts[0])
	if parts[1] == "no other bags." {
		return
	}
	for _, childStr := range strings.Split(parts[1], ", ") {
		addChild(bag, childStr, getOrMakeBag)
	}
}

func addChild(bag *Bag, childStr string, getOrMakeBag func(string) *Bag) {
	childParts := strings.Split(childStr, " ")
	childCount, _ := strconv.Atoi(childParts[0])
	child := getOrMakeBag(childParts[1] + " " + childParts[2])
	bag.Counts = append(bag.Counts, childCount)
	bag.Children = append(bag.Children, child)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
