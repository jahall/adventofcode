package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Conway cube
type Cube interface {
	Neighbours() []Cube
}

type Cube3d struct {
	x, y, z int
}

func (c Cube3d) Neighbours() []Cube {
	neighbours := make([]Cube, 26)
	i := 0
	rng := []int{-1, 0, 1}
	for _, combo := range product(rng, rng, rng) {
		dx := combo[0]
		dy := combo[1]
		dz := combo[2]
		if dx == 0 && dy == 0 && dz == 0 {
			continue
		}
		neighbours[i] = Cube3d{c.x + dx, c.y + dy, c.z + dz}
		i++
	}
	return neighbours
}

type Cube4d struct {
	x, y, z, w int
}

func (c Cube4d) Neighbours() []Cube {
	neighbours := make([]Cube, 80)
	i := 0
	rng := []int{-1, 0, 1}
	for _, combo := range product(rng, rng, rng, rng) {
		dx := combo[0]
		dy := combo[1]
		dz := combo[2]
		dw := combo[3]
		if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
			continue
		}
		neighbours[i] = Cube4d{c.x + dx, c.y + dy, c.z + dz, c.w + dw}
		i++
	}
	return neighbours
}

// NOTE: this and the use of it is not very pretty at all...but allowed
// me to learn a bit about variadic functions :)
func product(nums ...[]int) [][]int {
	if len(nums) == 0 {
		return [][]int{make([]int, 0)} // slice containing one empty slice
	}
	var prod [][]int
	for _, tail := range product(nums[1:]...) {
		for _, num := range nums[0] {
			newNums := []int{num}
			newNums = append(newNums, tail...)
			prod = append(prod, newNums)
		}
	}
	return prod
}

// Struct to house the pocket dimension
type PocketDimension struct {
	active map[Cube]bool
}

func LoadPocketDimension(test bool, dims int) *PocketDimension {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day17"+suffix+".txt"))
	file, _ := os.Open(path)
	active := make(map[Cube]bool)
	scanner := bufio.NewScanner(file)
	y := 0
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		for x, c := range line {
			if string(c) == "#" {
				switch dims {
				case 3:
					active[Cube3d{x: x, y: y}] = true
				case 4:
					active[Cube4d{x: x, y: y}] = true
				}
			}
		}
		y++
	}
	return &PocketDimension{active}
}

func (p *PocketDimension) NumActiveCubes() int {
	return len(p.active)
}

// Execute a single update cycle
func (p *PocketDimension) Cycle() {
	nextActive := make(map[Cube]bool)
	reviewed := make(map[Cube]bool)
	for activeCube := range p.active {
		for _, cube := range activeCube.Neighbours() {
			if !reviewed[cube] {
				activeNeighbours := p.numActiveNeighbours(cube)
				if p.active[cube] && (activeNeighbours == 2 || activeNeighbours == 3) {
					nextActive[cube] = true
				} else if !p.active[cube] && activeNeighbours == 3 {
					nextActive[cube] = true
				}
				reviewed[cube] = true
			}
		}
	}
	p.active = nextActive
}

func (p *PocketDimension) numActiveNeighbours(cube Cube) int {
	n := 0
	for _, neighbour := range cube.Neighbours() {
		if p.active[neighbour] {
			n++
		}
	}
	return n
}

func part1(pd *PocketDimension) {
	for i := 0; i < 6; i++ {
		pd.Cycle()
	}
	fmt.Printf("PART 1: %d active 3d cubes after 6 cycles\n", pd.NumActiveCubes())
}

func part2(pd *PocketDimension) {
	for i := 0; i < 6; i++ {
		pd.Cycle()
	}
	fmt.Printf("PART 2: %d active 4d cubes after 6 cycles\n", pd.NumActiveCubes())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	part1(LoadPocketDimension(test, 3))
	part2(LoadPocketDimension(test, 4))
}
