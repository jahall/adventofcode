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

func (notes *Notes) ScanningErrorRate() int {
	errorRate := 0
	for _, ticket := range notes.nearbyTickets {
		for _, value := range ticket.values {
			invalid := true
			for _, field := range notes.ticketFields {
				if field.IsValid(value) {
					invalid = false
				}
			}
			if invalid {
				errorRate += value
			}
		}
	}
	return errorRate
}

func part1(notes *Notes) {
	fmt.Printf("PART 1: Scanning error rate is %d\n", notes.ScanningErrorRate())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	notes := LoadNotes(test)
	part1(notes)
	//part2(notes)
}
