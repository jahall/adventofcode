package main

import (
	"fmt"
	"os"
	"strconv"
)

// The cup game
type Cups struct {
	cups []int
}

func LoadCups(test bool) *Cups {
	if test {
		return &Cups{cups: []int{3, 8, 9, 1, 2, 5, 4, 6, 7}}
	}
	return &Cups{cups: []int{3, 6, 4, 2, 9, 7, 5, 8, 1}}
}

func (c *Cups) Move() {
	// 1. Extract 3 cups
	n := len(c.cups)
	popped := c.cups[1:4]
	cups := make([]int, 0, n-3)
	cups = append(cups, c.cups[0])
	cups = append(cups, c.cups[4:]...)
	// 2. Find destination index
	destination := 0
	label := cups[0]
	for {
		label--
		if label < 1 {
			label = 9
		}
		for i, cup := range cups {
			if cup == label {
				destination = i
				break
			}
		}
		if destination > 0 {
			break
		}
	}
	// 3. Insert extracted cups
	newCups := make([]int, 0, n)
	newCups = append(newCups, cups[:destination+1]...)
	newCups = append(newCups, popped...)
	newCups = append(newCups, cups[destination+1:]...)
	// 4. Shift view left
	copy(c.cups[:n-1], newCups[1:])
	copy(c.cups[n-1:], newCups[:1])
}

// Cup labels working around clockwise after cup 1
func (c *Cups) Status() string {
	var index int
	for i, cup := range c.cups {
		if cup == 1 {
			index = i
		}
	}
	n := len(c.cups)
	arr := make([]int, 0, n-1)
	arr = append(arr, c.cups[index+1:]...)
	arr = append(arr, c.cups[:index]...)
	status := ""
	for _, cup := range arr {
		status += strconv.Itoa(cup)
	}
	return status
}

func part1(cups *Cups) {
	for i := 0; i < 100; i++ {
		cups.Move()
	}
	fmt.Printf("PART 1: Status after 100 moves is %s\n", cups.Status())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	part1(LoadCups(test))
}
