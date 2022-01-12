package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	data := loadData()
	part1(data)
	part2(data)
}

func part1(data []string) {
	trees := traverse(data, 1, 3)
	fmt.Printf("PART 1: Encountered %d trees\n", trees)
}

func part2(data []string) {
	n1 := traverse(data, 1, 1)
	n2 := traverse(data, 1, 3)
	n3 := traverse(data, 1, 5)
	n4 := traverse(data, 1, 7)
	n5 := traverse(data, 2, 1)
	fmt.Printf(
		"PART 1: Encountered %d x %d x %d x %d x %d = %d trees\n",
		n1, n2, n3, n4, n5, n1*n2*n3*n4*n5,
	)
}

func traverse(data []string, drow int, dcol int) int {
	trees := 0
	row, col := 0, 0
	for row < len(data) {
		if data[row][col:col+1] == "#" {
			trees++
		}
		row += drow
		col = (col + dcol) % len(data[0])
	}
	return trees
}

func loadData() []string {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day3.txt")
	check(err)
	scanner := bufio.NewScanner((file))
	var grid []string
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		grid = append(grid, line)
	}
	return grid
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
