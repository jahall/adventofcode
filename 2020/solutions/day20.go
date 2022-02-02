package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Tile
type Tile struct {
	id     int
	pixels [][]string
}

func (t *Tile) PossibleEdges() []string {
	return []string{
		toString(t.row(0), false),
		toString(t.row(0), true),
		toString(t.row(-1), false),
		toString(t.row(-1), true),
		toString(t.column(0), false),
		toString(t.column(0), true),
		toString(t.column(-1), false),
		toString(t.column(-1), true),
	}
}

func (t *Tile) Edge(side string) string {
	switch side {
	case "left":
		return toString(t.column(0), false)
	case "top":
		return toString(t.row(0), false)
	case "right":
		return toString(t.column(-1), false)
	default:
		return toString(t.row(-1), false)
	}
}

func (t *Tile) Flip() {
	res := len(t.pixels)
	pixels := make([][]string, res)
	for i := 0; i < res; i++ {
		pixels[i] = make([]string, res)
		for j := 0; j < res; j++ {
			pixels[i][j] = t.pixels[i][res-j-1]
		}
	}
	t.pixels = pixels
}

func (t *Tile) Rotate() {
	res := len(t.pixels)
	pixels := make([][]string, res)
	for i := 0; i < res; i++ {
		pixels[i] = make([]string, res)
		for j := 0; j < res; j++ {
			pixels[i][j] = t.pixels[j][res-i-1]
		}
	}
	t.pixels = pixels
}

func (t *Tile) NumHashPixels() int {
	count := 0
	for _, row := range t.pixels {
		for _, pix := range row {
			if pix == "#" {
				count++
			}
		}
	}
	return count
}

func toString(chars []string, reverse bool) string {
	str := ""
	for i := 0; i < len(chars); i++ {
		ii := i
		if reverse {
			ii = len(chars) - i - 1
		}
		str += chars[ii]
	}
	return str
}

func (t *Tile) row(index int) []string {
	if index < 0 {
		index = len(t.pixels) + index
	}
	return t.pixels[index]
}

func (t *Tile) column(index int) []string {
	res := len(t.pixels)
	if index < 0 {
		index = res + index
	}
	col := make([]string, res)
	for i := 0; i < res; i++ {
		col[i] = t.pixels[i][index]
	}
	return col
}

// Image
type Image struct {
	tiles       []*Tile
	edgeMap     map[string]map[int]bool
	unscrambled *Tile
}

func LoadImage(test bool) *Image {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day20"+suffix+".txt"))
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	image := Image{}
	tile := &Tile{}
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if strings.HasPrefix(line, "Tile") {
			if tile.id != 0 {
				image.tiles = append(image.tiles, tile)
			}
			tile = &Tile{}
			id, _ := strconv.Atoi(line[5 : len(line)-1])
			tile.id = id
		} else if line != "" {
			var row []string
			for _, char := range line {
				row = append(row, string(char))
			}
			tile.pixels = append(tile.pixels, row)
		}
	}
	image.tiles = append(image.tiles, tile)
	return &image
}

// Water roughness is the number of # symbols minus any that belong to a monster
func (im *Image) WaterRoughness() int {
	roughness := im.unscrambled.NumHashPixels()
	roughness -= im.CountMonsters() * 15
	return roughness
}

// Orient image to the monsters i.e. flip and rotate until we find a monster
func (im *Image) OrientToMonsters() {
	for flip := 0; flip < 2; flip++ {
		for rotate := 0; rotate < 4; rotate++ {
			if im.CountMonsters() > 0 {
				return
			}
			im.unscrambled.Rotate()
		}
		im.unscrambled.Flip()
	}
}

// Count the monsters in the unscrambled image
func (im *Image) CountMonsters() int {
	count := 0
	pix := im.unscrambled.pixels
	rows, cols := len(pix), len(pix[0])
	for i := 0; i < rows-3; i++ {
		for j := 0; j < cols-20; j++ {
			if (pix[i+1][j] == "#") && // tail
				(pix[i+2][j+1] == "#") &&
				(pix[i+2][j+4] == "#") && // last hump
				(pix[i+1][j+5] == "#") &&
				(pix[i+1][j+6] == "#") &&
				(pix[i+2][j+7] == "#") &&
				(pix[i+2][j+10] == "#") && // first hump
				(pix[i+1][j+11] == "#") &&
				(pix[i+1][j+12] == "#") &&
				(pix[i+2][j+13] == "#") &&
				(pix[i+2][j+16] == "#") && // head
				(pix[i+1][j+17] == "#") &&
				(pix[i+1][j+18] == "#") &&
				(pix[i+1][j+19] == "#") &&
				(pix[i][j+18] == "#") {
				count++
			}
		}
	}
	return count
}

// Unscramble the image
func (im *Image) Unscramble() {
	var grid [][]*Tile
	tileMap := im.tileMap()
	edgeMap := im.getEdgeMap()
	firstTile := im.getFirstTile()
	prevTile, matchSide, prevMatchSide := firstTile, "left", "right"
	grid = append(grid, []*Tile{firstTile})
	fmt.Printf("Added tile %d to position 0, 0\n", firstTile.id)
	row, matched := 0, 1
	for {
		edge := prevTile.Edge(prevMatchSide)
		otherTiles := edgeMap[edge]
		if len(otherTiles) == 1 {
			// Reached the end of a row
			prevTile, matchSide, prevMatchSide = grid[row][0], "top", "bottom"
			grid = append(grid, []*Tile{})
			row++
		} else {
			// There is an available match
			if len(otherTiles) > 2 {
				panic("Found multiple possible matches...")
			}
			var tile *Tile
			for tileId := range otherTiles {
				if tileId != prevTile.id {
					tile = tileMap[tileId]
				}
			}
			im.orientToMatch(tile, matchSide, edge)
			grid[row] = append(grid[row], tile)
			prevTile, matchSide, prevMatchSide = tile, "left", "right"
			matched++
			fmt.Printf("Added tile %d to position %d, %d\n", tile.id, row, len(grid[row])-1)
			if matched == len(im.tiles) {
				break
			}
		}
	}
	im.unscrambled = im.combineTiles(grid)
}

// Pick a corner and orient to be in top left
func (im *Image) getFirstTile() *Tile {
	corner := im.FindCorners()[0]
	edgeMap := im.getEdgeMap()
	for {
		leftUnique := len(edgeMap[corner.Edge("left")]) == 1
		topUnique := len(edgeMap[corner.Edge("top")]) == 1
		if leftUnique && topUnique {
			break
		}
		corner.Rotate()
	}
	return corner
}

// Flip and rotate the tile until the side matches the edge
func (im *Image) orientToMatch(tile *Tile, side string, edge string) {
	for flip := 0; flip < 2; flip++ {
		for rotate := 0; rotate < 4; rotate++ {
			if tile.Edge(side) == edge {
				return
			}
			tile.Rotate()
		}
		tile.Flip()
	}
}

// Combine a grid of tiles into a new single tile
func (im *Image) combineTiles(grid [][]*Tile) *Tile {
	pixels := [][]string{}
	for _, tileRow := range grid {
		for r := 1; r < 9; r++ {
			pixRow := []string{}
			for _, tile := range tileRow {
				pixRow = append(pixRow, tile.pixels[r][1:9]...)
			}
			pixels = append(pixels, pixRow)
		}
	}
	return &Tile{pixels: pixels}
}

// Corners have two unique edges
func (im *Image) FindCorners() []*Tile {
	uniqueEdgeCounts := make(map[int]int)
	for _, tileIdSet := range im.getEdgeMap() {
		if len(tileIdSet) == 1 {
			for tileId := range tileIdSet {
				uniqueEdgeCounts[tileId]++
			}
		}
	}
	var corners []*Tile
	tileMap := im.tileMap()
	for tileId, count := range uniqueEdgeCounts {
		// Assumes edges are not palendromes!
		if count >= 4 {
			corners = append(corners, tileMap[tileId])
		}
	}
	return corners
}

func (im *Image) tileMap() map[int]*Tile {
	tileMap := make(map[int]*Tile)
	for _, tile := range im.tiles {
		tileMap[tile.id] = tile
	}
	return tileMap
}

// Mapping from edge signature to set of tile ids with that signature
func (im *Image) getEdgeMap() map[string]map[int]bool {
	if im.edgeMap == nil {
		im.edgeMap = make(map[string]map[int]bool)
		for _, tile := range im.tiles {
			for _, edge := range tile.PossibleEdges() {
				if im.edgeMap[edge] == nil {
					im.edgeMap[edge] = make(map[int]bool)
				}
				im.edgeMap[edge][tile.id] = true
			}
		}
	}
	return im.edgeMap
}

func part1(image *Image) {
	prod := 1
	for _, corner := range image.FindCorners() {
		prod *= corner.id
	}
	fmt.Printf("PART 1: Product of corners is %d\n", prod)
}

func part2(image *Image) {
	fmt.Println("Unscrambling")
	image.Unscramble()
	fmt.Println("Orienting")
	image.OrientToMonsters()
	fmt.Printf("PART 2: Water roughness is %d\n", image.WaterRoughness())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	image := LoadImage(test)
	part1(image)
	part2(image)
}
