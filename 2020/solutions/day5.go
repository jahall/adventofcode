package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strings"
)

// Struct for seat identifier
type Seat struct {
	row int
	col int
}

func NewSeat(code string) Seat {
	row := toInt(code[:7], "B")
	col := toInt(code[7:], "R")
	return Seat{row: row, col: col}
}

func (s *Seat) id() int {
	return s.row*8 + s.col
}

func toInt(id string, posVal string) int {
	val := 0
	n := len(id)
	for i, char := range id {
		if string(char) == posVal {
			val += int(math.Pow(2, float64(n-i-1)))
		}
	}
	return val
}

func main() {
	seats := loadSeats()
	part1(seats)
	part2(seats)
}

func part1(seats []Seat) {
	maxId := 0
	for _, seat := range seats {
		id := seat.id()
		if id > maxId {
			maxId = id
		}
	}
	fmt.Printf("PART 1: Largest seat id is %d\n", maxId)
}

func part2(seats []Seat) {
	minRow, maxRow := findMinMaxRows(seats)
	emptySeats := findEmptySeats(seats, minRow, maxRow)
	for id, seat := range emptySeats {
		fmt.Printf("PART 2: Your seat is %+v with id %d\n", seat, id)
	}
}

func findMinMaxRows(seats []Seat) (int, int) {
	min, max := -1, -1
	for _, s := range seats {
		row := s.row
		if min == -1 || row < min {
			min = row
		}
		if max == -1 || row > max {
			max = row
		}
	}
	return min, max
}

func findEmptySeats(seats []Seat, minRow int, maxRow int) map[int]Seat {
	emptySeats := make(map[int]Seat)
	for row := minRow + 1; row < maxRow; row++ {
		for col := 0; col < 8; col++ {
			seat := Seat{row: row, col: col}
			emptySeats[seat.id()] = seat
		}
	}
	for _, seat := range seats {
		delete(emptySeats, seat.id())
	}
	return emptySeats
}

func loadSeats() []Seat {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day5.txt")
	check(err)
	var seats []Seat
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		code := strings.TrimSpace(scanner.Text())
		seats = append(seats, NewSeat(code))
	}
	return seats
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
