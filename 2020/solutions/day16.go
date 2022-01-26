package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Simple range struct
type Range struct {
	lower, upper int
}

func ParseRange(line string) Range {
	parts := strings.Split(line, "-")
	lower, _ := strconv.Atoi(parts[0])
	upper, _ := strconv.Atoi(parts[1])
	return Range{lower, upper}
}

func (r *Range) IsValid(value int) bool {
	return value >= r.lower && value <= r.upper
}

// Struct to house a ticket field
type TicketField struct {
	name        string
	validRanges []Range
}

func ParseTicketField(line string) *TicketField {
	parts := strings.Split(line, ": ")
	name := parts[0]
	var ranges []Range
	for _, rngStr := range strings.Split(parts[1], " or ") {
		ranges = append(ranges, ParseRange(rngStr))
	}
	return &TicketField{name: name, validRanges: ranges}
}

func (tf *TicketField) IsValid(value int) bool {
	for _, rng := range tf.validRanges {
		if rng.IsValid(value) {
			return true
		}
	}
	return false
}

// Struct to house ticket values
type Ticket struct {
	values []int
}

func ParseTicket(line string) *Ticket {
	parts := strings.Split(line, ",")
	values := make([]int, len(parts))
	for i, part := range parts {
		values[i], _ = strconv.Atoi(part)
	}
	return &Ticket{values}
}

// Struct to house your notes
type Notes struct {
	ticketFields  []*TicketField
	yourTicket    *Ticket
	nearbyTickets []*Ticket
}

func LoadNotes(test bool) *Notes {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day16"+suffix+".txt"))
	file, _ := os.Open(path)
	notes := Notes{}
	section := 0
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			section++
			continue
		} else if line == "your ticket:" || line == "nearby tickets:" {
			continue
		}
		switch section {
		case 0:
			notes.ticketFields = append(notes.ticketFields, ParseTicketField(line))
		case 1:
			notes.yourTicket = ParseTicket(line)
		case 2:
			notes.nearbyTickets = append(notes.nearbyTickets, ParseTicket(line))
		}
	}
	return &notes
}

// Sum up values for all invalid fields
func (notes *Notes) ScanningErrorRate() int {
	errorRate := 0
	for _, ticket := range notes.nearbyTickets {
		errorRate += notes.ticketError(ticket)
	}
	return errorRate
}

func (notes *Notes) ticketError(ticket *Ticket) int {
	err := 0
	for _, value := range ticket.values {
		invalid := true
		for _, field := range notes.ticketFields {
			if field.IsValid(value) {
				invalid = false
			}
		}
		if invalid {
			err += value
		}
	}
	return err
}

// Find mapping from field to correct index
func (notes *Notes) CalcFieldToIndex() map[string]int {
	possibilities := notes.findPossibilities()
	numFields := len(notes.ticketFields)
	fieldToIndex := make(map[string]int)
	for len(fieldToIndex) < numFields {
		// 1. Find unique possibilities
		for name, indices := range possibilities {
			if len(indices) == 1 {
				for idx := range indices {
					fieldToIndex[name] = idx
				}
				delete(possibilities, name)
			}
		}
		// 2. Refine remaining possibilities
		for _, idx := range fieldToIndex {
			for name := range possibilities {
				delete(possibilities[name], idx)
			}
		}
	}
	return fieldToIndex
}

func (notes *Notes) findPossibilities() map[string]map[int]bool {
	possibilities := notes.initPossibilities()
	for _, ticket := range notes.nearbyTickets {
		if notes.ticketError(ticket) > 0 {
			continue
		}
		for _, field := range notes.ticketFields {
			for i, value := range ticket.values {
				if !field.IsValid(value) {
					delete(possibilities[field.name], i)
				}
			}
		}
	}
	return possibilities
}

func (notes *Notes) initPossibilities() map[string]map[int]bool {
	possibilities := make(map[string]map[int]bool)
	numFields := len(notes.ticketFields)
	for _, field := range notes.ticketFields {
		possibilities[field.name] = make(map[int]bool)
		for i := 0; i < numFields; i++ {
			possibilities[field.name][i] = true
		}
	}
	return possibilities
}

func part1(notes *Notes) {
	fmt.Printf("PART 1: Scanning error rate is %d\n", notes.ScanningErrorRate())
}

func part2(notes *Notes) {
	fieldToIndex := notes.CalcFieldToIndex()
	result := 1
	for name, idx := range fieldToIndex {
		if strings.HasPrefix(name, "departure") {
			result *= notes.yourTicket.values[idx]
		}
	}
	fmt.Printf("PART 2: Product of departure field values is %d\n", result)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	notes := LoadNotes(test)
	part1(notes)
	part2(notes)
}
