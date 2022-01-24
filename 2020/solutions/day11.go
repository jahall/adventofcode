package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Simple wrapper around a row and col
type Seat struct {
	row, col int
}

// Struct for the waiting area
type WaitingArea struct {
	rows, cols int
	seats      map[Seat]bool
	occupied   map[Seat]bool
}

func LoadWaitingArea(test bool) *WaitingArea {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day11"+suffix+".txt"))
	file, _ := os.Open(path)
	seats := make(map[Seat]bool)
	cols, row := 0, 0
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		cols = len(line)
		for col, char := range line {
			if string(char) == "L" {
				seats[Seat{row, col}] = true
			}
		}
		row++
	}
	rows := row + 1
	return NewWaitingArea(rows, cols, seats)
}

func NewWaitingArea(rows, cols int, seats map[Seat]bool) *WaitingArea {
	occupied := make(map[Seat]bool)
	return &WaitingArea{rows, cols, seats, occupied}
}

// Simulate waiting room to convergence and return step at which it converged
func (w *WaitingArea) Simulate(method string, neighbourLimit int) int {
	step := 0
	for {
		step++
		changed := w.Update(method, neighbourLimit)
		if !changed {
			return step - 1
		}
	}
}

// Apply one update to the waiting area and flag if stabilised
func (w *WaitingArea) Update(method string, neighbourLimit int) bool {
	changed := false
	newOccupied := make(map[Seat]bool)
	for seat := range w.seats {
		numOccupied := 0
		switch method {
		case "neighbours":
			numOccupied = w.numOccupiedNeighbours(seat)
		case "in-sight":
			numOccupied = w.numOccupiedWithinSight(seat)
		}
		if !w.occupied[seat] && numOccupied == 0 {
			newOccupied[seat] = true
		} else if w.occupied[seat] && numOccupied >= neighbourLimit {
			delete(newOccupied, seat)
		} else if w.occupied[seat] {
			newOccupied[seat] = true
		}
		if newOccupied[seat] != w.occupied[seat] {
			changed = true
		}
	}
	w.occupied = newOccupied
	return changed
}

func (w *WaitingArea) numOccupiedNeighbours(seat Seat) int {
	n := 0
	for row := seat.row - 1; row <= seat.row+1; row++ {
		for col := seat.col - 1; col <= seat.col+1; col++ {
			neighbour := Seat{row, col}
			if neighbour != seat && w.occupied[neighbour] {
				n++
			}
		}
	}
	return n
}

func (w *WaitingArea) numOccupiedWithinSight(seat Seat) int {
	n := 0
	for drow := -1; drow <= 1; drow++ {
		for dcol := -1; dcol <= 1; dcol++ {
			// ignore self
			if drow == 0 && dcol == 0 {
				continue
			}
			i := 0
			for {
				i++
				other := Seat{seat.row + i*drow, seat.col + i*dcol}
				isSeat := w.seats[other]
				if isSeat || !w.withinArea(other) {
					if w.occupied[other] {
						n++
					}
					break
				}
			}
		}
	}
	return n
}

func (w *WaitingArea) withinArea(seat Seat) bool {
	return seat.row >= 0 && seat.row < w.rows && seat.col >= 0 && seat.col < w.cols
}

func part1(w *WaitingArea) {
	step := w.Simulate("neighbours", 4)
	fmt.Printf("PART 1: Converged at step %d with %d seats occupied\n", step, len(w.occupied))
}

func part2(w *WaitingArea) {
	step := w.Simulate("in-sight", 5)
	fmt.Printf("PART 2: Converged at step %d with %d seats occupied\n", step, len(w.occupied))
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	part1(LoadWaitingArea(test))
	// NOTE: Be sure to reload waiting area rather than use a
	// modified one from part 1!
	part2(LoadWaitingArea(test))
}
