package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// Container for a bag, which can hold other bags
type Instruction struct {
	op  string
	arg int
}

// Determine the next index and accumulator value
func (i *Instruction) Apply(index int, acc int) (int, int) {
	switch i.op {
	case "acc":
		return index + 1, acc + i.arg
	case "jmp":
		return index + i.arg, acc
	case "nop":
		return index + 1, acc
	default:
		return index, acc
	}
}

// A program to run a bunch of instructions
type Program struct {
	instructions []*Instruction
}

// Run and indicate whether it was successful
func (p *Program) Run() (int, bool) {
	index, acc := 0, 0
	visited := make(map[int]bool)
	for {
		visited[index] = true
		inst := p.instructions[index]
		index, acc = inst.Apply(index, acc)
		if index == len(p.instructions) {
			return acc, true
		}
		if visited[index] || index < 0 || index > len(p.instructions) {
			return acc, false
		}
	}
}

func main() {
	program := loadProgram()
	part1(program)
	part2(program)
}

func part1(program *Program) {
	acc, _ := program.Run()
	fmt.Printf("PART 1: Accumulator with the broken program is %d\n", acc)
}

func part2(program *Program) {
	for _, inst := range program.instructions {
		if inst.op == "acc" {
			continue
		}
		switchJmpNop(inst)
		acc, ok := program.Run()
		if ok {
			fmt.Printf("PART 2: Accumulator with the fixed program is %d\n", acc)
			return
		}
		switchJmpNop(inst)
	}
}

func switchJmpNop(i *Instruction) {
	switch i.op {
	case "jmp":
		i.op = "nop"
	case "nop":
		i.op = "jmp"
	}
}

func loadProgram() *Program {
	file, err := os.Open("/Users/Joe/src/adventofcode/2020/data/day8.txt")
	check(err)
	var instructions []*Instruction
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), " ")
		op := parts[0]
		arg, _ := strconv.Atoi(parts[1][1:])
		if parts[1][:1] == "-" {
			arg = -arg
		}
		instructions = append(instructions, &Instruction{op: op, arg: arg})
	}
	return &Program{instructions: instructions}
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
