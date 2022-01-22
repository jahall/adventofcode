package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

// Struct to control access to memory
type Memory struct {
	memory  map[uint64]uint64
	onMask  uint64
	offMask uint64
}

func NewMemory() *Memory {
	return &Memory{memory: make(map[uint64]uint64)}
}

func (m *Memory) Update(address uint64, value uint64) {
	value = value | m.onMask
	value = value & m.offMask
	m.memory[address] = value
}

func (m *Memory) Total() uint64 {
	var total uint64
	for _, value := range m.memory {
		total += value
	}
	return total
}

// The operation interface
type Op interface {
	apply(*Memory)
	show()
}

// Update the memories input mask
type Mask struct {
	on  uint64
	off uint64
}

func (m Mask) apply(mem *Memory) {
	mem.onMask = m.on
	mem.offMask = m.off
}

func (m Mask) show() {
	fmt.Printf("%x\n", m.on)
	fmt.Printf("%x\n", m.off)
}

// Update memory
type Update struct {
	address uint64
	value   uint64
}

func (u Update) apply(mem *Memory) {
	mem.Update(u.address, u.value)
}

func (u Update) show() {
	fmt.Printf("%v\n", u)
}

func LoadOps(test bool) []Op {
	suffix := ""
	if test {
		suffix = "_test"
	}
	file, _ := os.Open("/Users/Joe/src/adventofcode/2020/data/day14" + suffix + ".txt")
	var ops []Op
	scanner := bufio.NewScanner((file))
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		parts := strings.Split(line, " = ")
		var op Op
		if parts[0] == "mask" {
			op = makeMask(parts)
		} else {
			op = makeUpdate(parts)
		}
		ops = append(ops, op)
	}
	return ops
}

func makeMask(parts []string) Mask {
	maskStr := parts[1]
	n := len(maskStr)
	var on, off uint64
	for i := 0; i < n; i++ {
		switch string(maskStr[n-1-i]) {
		case "1":
			on += pow2(i)
		case "0":
			off += pow2(i)
		}
	}
	return Mask{on, ^off}
}

func makeUpdate(parts []string) Update {
	addressStr := parts[0][4 : len(parts[0])-1]
	address, _ := strconv.ParseUint(addressStr, 10, 64)
	value, _ := strconv.ParseUint(parts[1], 10, 64)
	return Update{address, value}
}

func pow2(i int) uint64 {
	return uint64(math.Pow(2, float64(i)))
}

func part1(ops []Op) {
	mem := NewMemory()
	for _, op := range ops {
		//op.show()
		op.apply(mem)
	}
	fmt.Printf("PART 1: Total is %d\n", mem.Total())
}

func part2(ops []Op) {
	//mem := Memory{}
	fmt.Printf("PART 2: \n")
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	ops := LoadOps(test)
	part1(ops)
	part2(ops)
}
