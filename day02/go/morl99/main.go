package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	a()
}

type Element int32
type Outcome int32

const (
	Rock     Element = 1
	Paper            = 2
	Scissors         = 3
)
const (
	Win   Outcome = 6
	Draw          = 3
	Loose         = 0
)

type Round struct {
	Them Element;
	Outcome Outcome
}

var lettersThem = map[string]Element{
	"A": Rock,
	"B": Paper,
	"C": Scissors,
	
}
var lettersOutcome = map[string]Outcome{
	"X": Loose,
	"Y": Draw,
	"Z": Win,
}

var againstRock = map[Outcome]Element{
	Win: Paper,
	Draw: Rock,
	Loose: Scissors,
}

var againstPaper = map[Outcome]Element{
	Win: Scissors,
	Draw: Paper,
	Loose: Rock,
}

var againstScissors = map[Outcome]Element{
	Win: Rock,
	Draw: Scissors,
	Loose: Paper,
}

var against = map[Element]map[Outcome]Element{
	Rock:     againstRock,
	Paper:    againstPaper,
	Scissors: againstScissors,
}

func a() {
	puzzleInput, _ := ioutil.ReadFile("./input.txt")
	guide := parseInput(string(puzzleInput))
	score := caluclateScore(guide)
	fmt.Println(score)
}

func caluclateScore(guide []Round) int {
	score := 0
	for _, round := range guide {
		score += int(round.Outcome) + int(against[round.Them][round.Outcome])
	}
	return score
}

func parseInput(input string) []Round {
	rounds := []Round{}

	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			values := strings.Fields(line)
			round := Round{lettersThem[values[0]], lettersOutcome[values[1]]}
			rounds = append(rounds, round)
		}
	}
	return rounds
}
