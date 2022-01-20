package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Action struct {
	action string
	value  int
}

func LoadActions(test bool) []Action {
	suffix := ""
	if test {
		suffix = "_test"
	}
	file, _ := os.Open("/Users/Joe/src/adventofcode/2020/data/day12" + suffix + ".txt")
	var actions []Action
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		value, _ := strconv.Atoi(line[1:])
		actions = append(actions, Action{action: line[:1], value: value})
	}
	return actions
}

type Ship struct {
	x, y     int
	heading  int // note: heading=0 means facing east, 90=north, etc
	waypoint int
}

// Apply a series of actions
func (s *Ship) Moves(actions []Action) {
	for _, a := range actions {
		s.Move(a)
	}
}

// Apply a given action
func (s *Ship) Move(action Action) {
	switch action.action {
	case "N":
		s.y += action.value
	case "S":
		s.y -= action.value
	case "E":
		s.x += action.value
	case "W":
		s.x -= action.value
	case "L":
		s.heading = (s.heading + action.value) % 360
	case "R":
		s.heading = (s.heading - action.value + 360) % 360
	case "F":
		switch s.heading {
		case 0:
			s.x += action.value
		case 90:
			s.y += action.value
		case 180:
			s.x -= action.value
		case 270:
			s.y -= action.value
		}
	}
}

func part1(actions []Action) {
	ship := Ship{}
	ship.Moves(actions)
	manhatten := math.Abs(float64(ship.x)) + math.Abs(float64(ship.y))
	fmt.Printf("PART 1: Manhatten distance from origin is %d\n", int64(manhatten))
}

func part2(actions []Action) {
	fmt.Printf("PART 2: %d\n", 0)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	actions := LoadActions(test)
	part1(actions)
	part2(actions)
}
