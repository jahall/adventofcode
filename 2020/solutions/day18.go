package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Expression
type Expression struct {
	str string
}

func NewExpression(str string) Expression {
	str = strings.ReplaceAll(str, " ", "") + "$"
	return Expression{str}
}

func (e Expression) Eval(precedence string) int {
	value, _ := e.eval(precedence, 0)
	return value
}

// Returns value and new offset, assumes only numbers 0-9 will appear
func (e Expression) eval(precedence string, offset int) (int, int) {
	values := []int{}
	ops := []string{""}
	for {
		char := string(e.str[offset])
		if char == "$" || char == ")" {
			result := -1
			switch precedence {
			case "=":
				result = e.combineValuesSamePrecedence(values, ops)
			case "+*":
				result = e.combineValuesAdditionPrecedence(values, ops)
			}
			return result, offset
		}
		switch char {
		case "(":
			var value int
			value, offset = e.eval(precedence, offset+1)
			values = append(values, value)
		case "+", "*":
			ops = append(ops, char)
		default:
			value, _ := strconv.Atoi(char)
			values = append(values, value)
		}
		offset++
	}
}

func (e Expression) combineValuesSamePrecedence(values []int, ops []string) int {
	var result int
	for i, val := range values {
		switch ops[i] {
		case "":
			result = val
		case "+":
			result += val
		case "*":
			result *= val
		}
	}
	return result
}

func (e Expression) combineValuesAdditionPrecedence(values []int, ops []string) int {
	for i := range values {
		if ops[i] == "+" {
			values[i] = values[i] + values[i-1]
			values[i-1] = -1
		}
	}
	result := 1
	for _, val := range values {
		if val != -1 {
			result *= val
		}
	}
	return result
}

func LoadExpressions(test bool) []Expression {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day18"+suffix+".txt"))
	file, _ := os.Open(path)
	var expressions []Expression
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		expressions = append(expressions, NewExpression(line))
	}
	return expressions
}

// NOTE: Pointless use of worker pool...but mainly for my own understanding
func worker(jobs <-chan Expression, results chan<- int, precedence string) {
	for exp := range jobs {
		results <- exp.Eval(precedence)
	}
}

func evalAll(expressions []Expression, precedence string) int {
	n := len(expressions)
	jobs := make(chan Expression, n)
	results := make(chan int, n)
	for w := 1; w <= 10; w++ {
		go worker(jobs, results, precedence)
	}
	for _, exp := range expressions {
		jobs <- exp
	}
	close(jobs)
	result := 0
	for i := 0; i < n; i++ {
		result += <-results
	}
	return result
}

func part1(expressions []Expression) {
	result := evalAll(expressions, "=")
	fmt.Printf("PART 1: Sum of all expressions (with = precedence) is %d\n", result)
}

func part2(expressions []Expression) {
	result := evalAll(expressions, "+*")
	fmt.Printf("PART 2: Sum of all expressions (with +* precedence) is %d\n", result)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	expressions := LoadExpressions(test)
	part1(expressions)
	part2(expressions)
}
