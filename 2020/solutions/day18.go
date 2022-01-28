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
	if string(str[0]) != "(" {
		str = "(" + str + ")"
	}
	str = strings.ReplaceAll(str, " ", "")
	return Expression{str}
}

func (e Expression) Eval() int {
	value, _ := e.eval(0, 0)
	return value
}

// Returns value and new offset, assumes only numbers 0-9 will appear
func (e Expression) eval(offset int, depth int) (int, int) {
	value, op := 0, ""
	for {
		if offset >= len(e.str) {
			return value, offset
		}
		char := string(e.str[offset])
		subValue := -1
		switch char {
		case "(":
			subValue, offset = e.eval(offset+1, depth+1)
		case ")":
			return value, offset
		case "+", "*":
			op = char
		default:
			subValue, _ = strconv.Atoi(char)
		}
		if subValue > -1 {
			switch op {
			case "+":
				value += subValue
			case "*":
				value *= subValue
			default:
				value = subValue
			}
		}
		offset++
	}
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

func part1(expressions []Expression) {
	result := 0
	for _, e := range expressions {
		result += e.Eval()
	}
	fmt.Printf("PART 1: Sum of all expressions is %d\n", result)
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	expressions := LoadExpressions(test)
	part1(expressions)
}
