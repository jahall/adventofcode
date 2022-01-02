package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	data := loadData()
	part1(data)
	part2(data)
}

func part1(data []int) {
	for i, num1 := range data {
		for _, num2 := range data[i+1:] {
			if num1+num2 == 2020 {
				fmt.Printf("PART 1: %d + %d = 2020 (%d x %d = %d)\n",
					num1, num2, num1, num2, num1*num2)
				return
			}
		}
	}
}

func part2(data []int) {
	for i, num1 := range data {
		for j, num2 := range data[i+1:] {
			for _, num3 := range data[j+1:] {
				if num1+num2+num3 == 2020 {
					fmt.Printf("PART 2: %d + %d + %d = 2020 (%d x %d x %d = %d)\n",
						num1, num2, num3, num1, num2, num3, num1*num2*num3)
					return
				}
			}
		}
	}
}

func loadData() []int {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day1.txt")
	check(err)
	scanner := bufio.NewScanner((file))
	var numbers []int
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		check(err)
		numbers = append(numbers, num)
	}
	return numbers
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
