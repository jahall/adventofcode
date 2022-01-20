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

// Location
type Loc struct {
	x, y int
}

// Manhatten distance from the origin
func (loc *Loc) Manhatten() int {
	absX := math.Abs(float64(loc.x))
	absY := math.Abs(float64(loc.y))
	return int(absX + absY)
}

// Rotate location about the origin
func (loc *Loc) Rotate(direction string, angle int) {
	if direction == "R" {
		angle = -angle
	}
	angle = (angle + 360) % 360
	switch angle {
	case 0:
	case 90:
		loc.x, loc.y = -loc.y, loc.x
	case 180:
		loc.x, loc.y = -loc.x, -loc.y
	case 270:
		loc.x, loc.y = loc.y, -loc.x
	default:
		panic("Bad angle")
	}
}

// Ship
type Ship struct {
	position *Loc
	waypoint *Loc
	heading  int // note: heading=0 means facing east, 90=north, etc
}

func NewShip() *Ship {
	position := Loc{0, 0}
	waypoint := Loc{10, 1}
	return &Ship{position: &position, waypoint: &waypoint}
}

// Apply a series of actions
func (s *Ship) Moves(actions []Action, method string) {
	for _, a := range actions {
		switch method {
		case "naive":
			s.NaiveMove(a)
		case "correct":
			s.CorrectMove(a)
		}
	}
}

// Apply a given action
func (s *Ship) NaiveMove(action Action) {
	switch action.action {
	case "N", "S", "E", "W":
		s.moveLocation(action, s.position)
	case "L":
		s.heading = (s.heading + action.value) % 360
	case "R":
		s.heading = (s.heading - action.value + 360) % 360
	case "F":
		switch s.heading {
		case 0:
			s.position.x += action.value
		case 90:
			s.position.y += action.value
		case 180:
			s.position.x -= action.value
		case 270:
			s.position.y -= action.value
		}
	}
}

// Apply a given action...correctly
func (s *Ship) CorrectMove(action Action) {
	switch action.action {
	case "N", "S", "E", "W":
		s.moveLocation(action, s.waypoint)
	case "L", "R":
		s.waypoint.Rotate(action.action, action.value)
	case "F":
		s.position.x += action.value * s.waypoint.x
		s.position.y += action.value * s.waypoint.y
	}
}

func (s *Ship) moveLocation(action Action, loc *Loc) {
	switch action.action {
	case "N":
		loc.y += action.value
	case "S":
		loc.y -= action.value
	case "E":
		loc.x += action.value
	case "W":
		loc.x -= action.value
	}
}

func part1(actions []Action) {
	ship := NewShip()
	ship.Moves(actions, "naive")
	fmt.Printf("PART 1: Manhatten distance from origin is %d\n", ship.position.Manhatten())
}

func part2(actions []Action) {
	ship := NewShip()
	ship.Moves(actions, "correct")
	fmt.Printf("PART 2: Manhatten distance from origin is %d\n", ship.position.Manhatten())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	actions := LoadActions(test)
	part1(actions)
	part2(actions)
}
