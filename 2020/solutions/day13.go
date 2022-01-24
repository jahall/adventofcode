package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type Bus struct {
	id       int
	position int
}

func (b *Bus) Departing(timestamp int) bool {
	return timestamp%b.id == 0
}

type Scenario struct {
	arrival int
	buses   []*Bus
}

func LoadScenario(test bool) *Scenario {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day13"+suffix+".txt"))
	file, _ := os.Open(path)
	scenario := Scenario{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if scenario.arrival == 0 {
			scenario.arrival, _ = strconv.Atoi(line)
		} else {
			for pos, idStr := range strings.Split(line, ",") {
				if idStr == "x" {
					continue
				}
				id, _ := strconv.Atoi(idStr)
				scenario.buses = append(scenario.buses, &Bus{id, pos})
			}
			break
		}
	}
	return &scenario
}

// Find earliest departure time
func (s *Scenario) EarliestDeparture() (*Bus, int) {
	wait := 0
	for {
		for _, bus := range s.buses {
			if bus.Departing(s.arrival + wait) {
				return bus, wait
			}
		}
		wait++
	}
}

// Find earliest departure time matching their position
func (s *Scenario) DepartureTimeMatchingPositions() int {
	timestamp := 0
	increment := s.buses[0].id
	for _, bus := range s.buses[1:] {
		for {
			if (timestamp+bus.position)%bus.id == 0 {
				// NOTE: it was very painful figuring out and coming to the
				// conclusion that the increments can be increased multiplicatively
				// and slowly piecing together this function - had to do it
				// in Python first! ...assuming the fact the bus ids are prime
				// numbers is somehow important...

				// NOTE 2: Apparently something to do with Chinese Remainder Theorem
				// https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/
				increment *= bus.id
				break
			}
			timestamp += increment
		}
	}
	return timestamp
}

func part1(scenario *Scenario) {
	bus, wait := scenario.EarliestDeparture()
	fmt.Printf("PART 1: Depart on bus %+v after %d mins = %d\n", *bus, wait, bus.id*wait)
}

func part2(scenario *Scenario) {
	timestamp := scenario.DepartureTimeMatchingPositions()
	fmt.Printf("PART 2: Silly departure time is %d\n", timestamp)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	scenario := LoadScenario(test)
	part1(scenario)
	part2(scenario)
}
