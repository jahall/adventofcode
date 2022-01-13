package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// Struct for passport fields
type Passport struct {
	fields map[string]string
}

func makePassport() Passport {
	// https://stackoverflow.com/questions/18125625/constructors-in-go
	p := Passport{fields: make(map[string]string)}
	return p
}

func (p *Passport) isStructureValid() bool {
	requiredFields := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	for _, field := range requiredFields {
		if _, ok := p.fields[field]; !ok {
			return false
		}
	}
	return true
}

func (p *Passport) isValid() bool {
	if !p.isStructureValid() {
		return false
	}

	checkNumInRange := func(field string, lower int, upper int) bool {
		num, err := atoi(p.fields[field])
		return err == nil && num >= lower && num <= upper
	}

	// 1. Birth year
	if !checkNumInRange("byr", 1920, 2002) {
		return false
	}
	// 2. Issue year
	if !checkNumInRange("iyr", 2010, 2020) {
		return false
	}
	// 3. Expiration year
	if !checkNumInRange("eyr", 2020, 2030) {
		return false
	}
	// 4. Height
	hgt := p.fields["hgt"]
	num, err := atoi(hgt[:len(hgt)-2])
	suffix := hgt[len(hgt)-2:]
	switch suffix {
	case "cm":
		if err != nil || num < 150 || num > 193 {
			return false
		}
	case "in":
		if err != nil || num < 59 || num > 76 {
			return false
		}
	default:
		return false
	}
	// 5. Hair color
	match, _ := regexp.MatchString("^#[0-9a-f]{6,6}$", p.fields["hcl"])
	if !match {
		return false
	}
	// 6. Eye color
	switch p.fields["ecl"] {
	case "amb", "blu", "brn", "gry", "grn", "hzl", "oth":
		{
		}
	default:
		return false
	}
	// 7. Passport id
	match, _ = regexp.MatchString("^[0-9]{9,9}$", p.fields["pid"])
	if !match {
		return false
	}
	return true
}

func main() {
	part1()
	part2()
}

func part1() {
	channel := make(chan Passport)
	go loadPassports(channel)
	nValid := 0
	for passport := range channel {
		if passport.isStructureValid() {
			nValid++
		}
	}
	fmt.Printf("PART 1: %d passports with valid structures\n", nValid)
}

func part2() {
	channel := make(chan Passport)
	go loadPassports(channel)
	nValid := 0
	for passport := range channel {
		if passport.isValid() {
			nValid++
		}
	}
	fmt.Printf("PART 2: %d valid passwords\n", nValid)
}

func loadPassports(channel chan<- Passport) {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day4.txt")
	check(err)
	scanner := bufio.NewScanner((file))
	passport := makePassport()
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			channel <- passport
			passport = makePassport()
			continue
		}
		for _, part := range strings.Split(line, " ") {
			kv := strings.Split(part, ":")
			passport.fields[kv[0]] = kv[1]
		}
	}
	channel <- passport
	close(channel)
}

func atoi(str string) (int, error) {
	val, err := strconv.Atoi(str)
	return val, err
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
