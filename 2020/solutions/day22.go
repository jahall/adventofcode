package main

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// Deck of cards
type Deck struct {
	cards []int
}

func NewDeck() *Deck {
	return &Deck{cards: make([]int, 0, 1000)}
}

func (d *Deck) Draw() int {
	card := d.cards[0]
	d.cards = d.cards[1:]
	return card
}

func (d *Deck) Add(cards ...int) *Deck {
	for _, card := range cards {
		d.cards = append(d.cards, card)
	}
	return d
}

func (d *Deck) Size() int {
	return len(d.cards)
}

func (d *Deck) Score() int {
	score := 0
	for i, card := range d.cards {
		mul := len(d.cards) - i
		score += mul * card
	}
	return score
}

func (d *Deck) SubDeck(cutoff int) *Deck {
	sub := NewDeck()
	for i := 0; i < cutoff; i++ {
		sub.Add(d.cards[i])
	}
	return sub
}

func (d *Deck) Id() string {
	id := ""
	for _, card := range d.cards {
		id += "," + strconv.Itoa(card)
	}
	return strings.TrimLeft(id, ",")
}

// A game is played and returns the winner and the winning deck
type Game interface {
	Play() (int, *Deck)
}

// Standard combat
type Combat struct {
	deck1, deck2 *Deck
}

func NewCombat(deck1 *Deck, deck2 *Deck) *Combat {
	return &Combat{deck1, deck2}
}

func (g *Combat) Play() (int, *Deck) {
	for g.deck1.Size() > 0 && g.deck2.Size() > 0 {
		card1 := g.deck1.Draw()
		card2 := g.deck2.Draw()
		if card1 > card2 {
			g.deck1.Add(card1, card2)
		} else {
			g.deck2.Add(card2, card1)
		}
	}
	if g.deck1.Size() > 0 {
		return 1, g.deck1
	} else {
		return 2, g.deck2
	}
}

// Recursive combat
type RecursiveCombat struct {
	deck1, deck2 *Deck
	played       map[string]bool
}

func NewRecursiveCombat(deck1 *Deck, deck2 *Deck) *RecursiveCombat {
	played := make(map[string]bool)
	return &RecursiveCombat{deck1, deck2, played}
}

func (g *RecursiveCombat) Play() (int, *Deck) {
	for g.deck1.Size() > 0 && g.deck2.Size() > 0 {
		// If either deck has been played before, player 1 wins
		deck1Played := g.checkPlayed("1", g.deck1)
		deck2Played := g.checkPlayed("2", g.deck2)
		if deck1Played || deck2Played {
			return 1, g.deck1
		}
		// Otherwise, both draw cards
		card1 := g.deck1.Draw()
		card2 := g.deck2.Draw()
		// If there are sufficient cards, play a sub-game
		var winner int
		if g.deck1.Size() >= card1 && g.deck2.Size() >= card2 {
			subDeck1 := g.deck1.SubDeck(card1)
			subDeck2 := g.deck2.SubDeck(card2)
			game := NewRecursiveCombat(subDeck1, subDeck2)
			winner, _ = game.Play()
		} else if card1 > card2 {
			// If not, winner has highest card
			winner = 1
		} else {
			winner = 2
		}
		if winner == 1 {
			g.deck1.Add(card1, card2)
		} else {
			g.deck2.Add(card2, card1)
		}
	}
	if g.deck1.Size() > 0 {
		return 1, g.deck1
	} else {
		return 2, g.deck2
	}
}

func (g *RecursiveCombat) checkPlayed(prefix string, deck *Deck) bool {
	key := prefix + ":" + deck.Id()
	played := g.played[key]
	g.played[key] = true
	return played
}

// NOTE: Don't specify return type as a pointer to an interface!!
// A pointer to a concrete struct still satisfies the interface
// definition so all will be ok (this caused me problems in previous days)
func LoadGame(test bool, recursive bool) Game {
	suffix := ""
	if test {
		suffix = "_test"
	}
	path, _ := filepath.Abs(filepath.Join("data", "day22"+suffix+".txt"))
	file, _ := os.Open(path)
	scanner := bufio.NewScanner(file)
	deck1, deck2 := NewDeck(), NewDeck()
	deck := deck1
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if strings.HasPrefix(line, "Player") {
			continue
		}
		if line == "" {
			deck = deck2
			continue
		}
		card, _ := strconv.Atoi(line)
		deck.Add(card)
	}
	if !recursive {
		return NewCombat(deck1, deck2)
	} else {
		return NewRecursiveCombat(deck1, deck2)
	}
}

func part1(game Game) {
	winner, deck := game.Play()
	fmt.Printf("PART 1: Player %d wins with score of %d\n", winner, deck.Score())
}

func part2(game Game) {
	winner, deck := game.Play()
	fmt.Printf("PART 2: Player %d wins with score of %d\n", winner, deck.Score())
}

func main() {
	test := len(os.Args[1:]) == 1 && os.Args[1] == "test"
	part1(LoadGame(test, false))
	part2(LoadGame(test, true))
}
