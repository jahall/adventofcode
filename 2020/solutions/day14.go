package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Struct to control access to memory
type Memory struct {
	version      int
	memory       map[uint64]uint64
	offMask      uint64
	onMask       uint64
	floatingMask uint64
}

func NewMemory(version int) *Memory {
	return &Memory{version: version, memory: make(map[uint64]uint64)}
}

func (m *Memory) Update(address uint64, value uint64) {
	if m.version == 1 {
		value = value & ^m.offMask
		value = value | m.onMask
		m.memory[address] = value

	} else if m.version == 2 {
		address = address | m.onMask
		address = address & ^m.floatingMask // turn off all floating bits for now
		for _, mask := range m.floatingMasks() {
			m.memory[address|mask] = value
		}
	}
}

func (m *Memory) floatingMasks() []uint64 {
	bits := m.toBits(m.floatingMask)
	return m.combinations(bits)
}

func (m *Memory) toBits(value uint64) []uint64 {
	var bits []uint64
	i := 0
	for value != 0 {
		if value&1 == 1 {
			bits = append(bits, pow2(i))
		}
		value >>= 1
		i++
	}
	return bits
}

func (m *Memory) combinations(bits []uint64) []uint64 {
	if len(bits) == 0 {
		return []uint64{0}
	}
	var combos []uint64
	for _, combo := range m.combinations(bits[1:]) {
		combos = append(combos, combo)
		combos = append(combos, bits[0]|combo)
	}
	return combos
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
	off      uint64
	on       uint64
	floating uint64
}

func (m Mask) apply(mem *Memory) {
	mem.offMask = m.off
	mem.onMask = m.on
	mem.floatingMask = m.floating
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
	path, _ := filepath.Abs(filepath.Join("data", "day14"+suffix+".txt"))
	file, _ := os.Open(path)
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
	var off, on, floating uint64
	for i := 0; i < n; i++ {
		switch string(maskStr[n-1-i]) {
		case "0":
			off += pow2(i)
		case "1":
			on += pow2(i)
		case "X":
			floating += pow2(i)
		}
	}
	return Mask{on: on, off: off, floating: floating}
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
	mem := NewMemory(1)
	for _, op := range ops {
		op.apply(mem)
	}
	fmt.Printf("PART 1: Total is %d\n", mem.Total())
}

func part2(ops []Op) {
	mem := NewMemory(2)
	for _, op := range ops {
		op.apply(mem)
	}
	fmt.Printf("PART 2: Total using v2 is %d\n", mem.Total())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	ops := LoadOps(test)
	part1(ops)
	part2(ops)
}
