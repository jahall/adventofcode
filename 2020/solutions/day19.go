package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Rule
type Rule interface {
	Matches(rules map[int]Rule) []string
}

// Simple leaf
type CharRule struct {
	char string
}

// NOTE: Interface methods don't like pointer receivers!
func (r CharRule) Matches(rules map[int]Rule) []string {
	return []string{r.char}
}

// Composite rule
type CompositeRule struct {
	possibilities [][]int
}

func (r CompositeRule) Matches(rules map[int]Rule) []string {
	allMatches := []string{}
	for _, subRuleIds := range r.possibilities {
		var newMatches []string
		prevMatches := []string{""}
		for _, id := range subRuleIds {
			newMatches = []string{}
			rule := rules[id]
			for _, head := range prevMatches {
				for _, tail := range rule.Matches(rules) {
					newMatches = append(newMatches, head+tail)
				}
			}
			prevMatches = newMatches
		}
		allMatches = append(allMatches, newMatches...)
	}
	return allMatches
}

type Rules struct {
	rules       map[int]Rule
	ruleMatches map[int][]string
}

func NewRules() *Rules {
	rules := make(map[int]Rule)
	ruleMatches := make(map[int][]string)
	return &Rules{rules, ruleMatches}
}

func (r *Rules) Update(line string) {
	parts := strings.Split(line, ": ")
	id, _ := strconv.Atoi(parts[0])
	line = parts[1]
	isChar := strings.HasPrefix(line, "\"")
	var rule Rule
	if isChar {
		rule = CharRule{char: strings.Trim(line, "\"")}
	} else {
		possibilities := [][]int{}
		for _, part := range strings.Split(line, " | ") {
			subIds := []int{}
			for _, idStr := range strings.Split(part, " ") {
				subId, _ := strconv.Atoi(idStr)
				subIds = append(subIds, subId)
			}
			possibilities = append(possibilities, subIds)
		}
		rule = CompositeRule{possibilities: possibilities}
	}
	r.rules[id] = rule
}

func (r *Rules) ValidMessage(message string) bool {
	ruleId := 0
	matches, exists := r.ruleMatches[ruleId]
	if !exists {
		// Cache previous results cos its slooooooow
		matches = r.rules[ruleId].Matches(r.rules)
		r.ruleMatches[ruleId] = matches
	}
	for _, match := range matches {
		if message == match {
			return true
		}
	}
	return false
}

// A valid message is a series of valid "42" strings followed by
// a shorter series of "31" messages
func (r *Rules) ValidMessageFixed(message string) bool {
	matches42, exists := r.ruleMatches[42]
	matches31, _ := r.ruleMatches[31]
	if !exists {
		matches42 = r.rules[42].Matches(r.rules)
		matches31 = r.rules[31].Matches(r.rules)
		r.ruleMatches[42] = matches42
		r.ruleMatches[31] = matches31
	}
	num42s, offset := r.findSubMatches(message, matches42, 0)
	num31s, offset := r.findSubMatches(message, matches31, offset)
	return num42s > 0 && num31s > 0 && num42s > num31s && offset == len(message)
}

func (r *Rules) findSubMatches(message string, matches []string, offset int) (int, int) {
	numMatches := 0
	for {
		foundMatch := false
		for _, match := range matches {
			end := min(offset+len(match), len(message))
			if message[offset:end] == match {
				numMatches++
				offset += len(match)
				foundMatch = true
				break
			}
		}
		if !foundMatch {
			break
		}
	}
	return numMatches, offset
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func LoadRulesAndMessages(test bool) (*Rules, []string) {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day19"+suffix+".txt"))
	file, _ := os.Open(path)
	rules := NewRules()
	messages := []string{}
	scanner := bufio.NewScanner(file)
	parseRules := true
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			parseRules = false
			continue
		}
		if parseRules {
			rules.Update(line)
		} else {
			messages = append(messages, line)
		}
	}
	return rules, messages
}

func part1(rules *Rules, messages []string) {
	count := 0
	for _, m := range messages {
		if rules.ValidMessage(m) {
			count++
		}
	}
	fmt.Printf("PART 1: %d valid messages\n", count)
}

func part2(rules *Rules, messages []string) {
	count := 0
	for _, m := range messages {
		if rules.ValidMessageFixed(m) {
			count++
		}
	}
	fmt.Printf("PART 2: %d valid messages based on fixed rules\n", count)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	rules, messages := LoadRulesAndMessages(test)
	part1(rules, messages)
	part2(rules, messages)
}
