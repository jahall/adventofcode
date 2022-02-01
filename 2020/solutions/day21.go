package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

// Handy set structure
type Set struct {
	members map[string]bool
}

func NewSet() *Set {
	return &Set{members: make(map[string]bool)}
}

func (s *Set) Contains(item string) bool {
	return s.members[item]
}

func (s *Set) Size() int {
	return len(s.members)
}

func (s *Set) Add(item string) *Set {
	s.members[item] = true
	return s
}

func (s *Set) Update(items []string) *Set {
	for _, item := range items {
		s.Add(item)
	}
	return s
}

func (s *Set) Discard(item string) *Set {
	delete(s.members, item)
	return s
}

// NOTE: Feels very "go" to do this op in-place
func (s *Set) Intersection(other *Set) *Set {
	for item := range s.members {
		if !other.Contains(item) {
			s.Discard(item)
		}
	}
	return s
}

func (s *Set) Subtract(other *Set) *Set {
	for item := range other.members {
		if s.Contains(item) {
			s.Discard(item)
		}
	}
	return s
}

func (s *Set) Union(other *Set) *Set {
	for item := range other.members {
		s.Add(item)
	}
	return s
}

func (s *Set) Members() []string {
	// https://stackoverflow.com/questions/13422578/in-golang-is-there-a-nice-way-of-getting-a-slice-of-values-from-a-map
	items := make([]string, 0, s.Size())
	for m := range s.members {
		items = append(items, m)
	}
	return items
}

// Food definition
type Food struct {
	ingredients []string
	allergens   []string
}

func LoadFoods(test bool) []*Food {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day21"+suffix+".txt"))
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	foods := []*Food{}
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		line = strings.TrimRight(line, ")")
		parts := strings.Split(line, " (contains ")
		food := Food{
			ingredients: strings.Split(parts[0], " "),
			allergens:   strings.Split(parts[1], ", "),
		}
		foods = append(foods, &food)
	}
	return foods
}

func findAllergenIngredient(foods []*Food) map[string]string {
	allergenIngredients := findPossibleAllergenIngredients(foods)
	for {
		// 1. Find unique mappings
		unique := NewSet()
		finished := true
		for _, ings := range allergenIngredients {
			if ings.Size() == 1 {
				unique.Union(ings)
			} else {
				finished = false
			}
		}
		// 2. If everything is unique, we're done
		if finished {
			break
		}
		// 3. Otherwise discard the already known ones
		for _, ings := range allergenIngredients {
			if ings.Size() > 1 {
				ings.Subtract(unique)
			}
		}
	}
	allergenIngredient := make(map[string]string)
	for allergen, ings := range allergenIngredients {
		allergenIngredient[allergen] = ings.Members()[0]
	}
	return allergenIngredient
}

func findPossibleAllergenIngredients(foods []*Food) map[string]*Set {
	possibilities := make(map[string]*Set)
	for _, food := range foods {
		for _, allergen := range food.allergens {
			foodIngs := NewSet().Update(food.ingredients)
			relatedIngs, seenBefore := possibilities[allergen]
			if seenBefore {
				relatedIngs.Intersection(foodIngs)
			} else {
				possibilities[allergen] = foodIngs
			}
		}
	}
	return possibilities
}

func part1(foods []*Food) {
	possibleAllergenIngredients := findPossibleAllergenIngredients(foods)
	suspectIngredients := NewSet()
	for _, ings := range possibleAllergenIngredients {
		suspectIngredients.Union(ings)
	}
	count := 0
	for _, food := range foods {
		for _, ing := range food.ingredients {
			if !suspectIngredients.Contains(ing) {
				count++
			}
		}
	}
	fmt.Printf("PART 1: Non-suspect ingredients appear %d times\n", count)
}

func part2(foods []*Food) {
	allergenIngredient := findAllergenIngredient(foods)
	allergens := make([]string, 0, len(allergenIngredient))
	for allergen := range allergenIngredient {
		allergens = append(allergens, allergen)
	}
	sort.Strings(allergens)
	canonicalList := ""
	for _, allergen := range allergens {
		canonicalList += "," + allergenIngredient[allergen]
	}
	fmt.Printf("PART 2: Canonical list of ingredients is %s\n", canonicalList[1:])
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	foods := LoadFoods(test)
	part1(foods)
	part2(foods)
}
