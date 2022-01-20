package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Bus struct {
	id int
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
	file, _ := os.Open("/Users/Joe/src/adventofcode/2020/data/day13" + suffix + ".txt")
	scenario := Scenario{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if scenario.arrival == 0 {
			scenario.arrival, _ = strconv.Atoi(line)
		} else {
			for _, idStr := range strings.Split(line, ",") {
				if idStr == "x" {
					continue
				}
				id, _ := strconv.Atoi(idStr)
				scenario.buses = append(scenario.buses, &Bus{id})
			}
		}
	}
	return &scenario
}

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

func part1(scenario *Scenario) {
	bus, wait := scenario.EarliestDeparture()
	fmt.Printf("PART 1: Depart on bus %+v after %d mins = %d\n", *bus, wait, bus.id*wait)
}

func part2(scenario *Scenario) {
	fmt.Printf("PART 2: Yo\n")
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	scenario := LoadScenario(test)
	part1(scenario)
	part2(scenario)
}
