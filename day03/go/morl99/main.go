package main

import (
	"bufio"
	"errors"
	"fmt"
	"io/ioutil"
	"strings"

	"github.com/thoas/go-funk"
)

func main() {
	a()
}

type Compartment []rune

type Rucksack struct {
	Left  Compartment
	Right Compartment
}

func a() {
	puzzleInput, _ := ioutil.ReadFile("./input.txt")
	rucksacks := parseInput(string(puzzleInput))
	sum := 0
	for _, rucksack := range rucksacks {
		duplicate, error := findDuplicateItem(rucksack)
		if error != nil {
			fmt.Println(error)
		} else {
			sum += calculatePriority(duplicate)
		}
	}
	fmt.Println(sum)
}

func findDuplicateItem(rucksack Rucksack) (rune, error) {
	for _, item := range rucksack.Left {
		if funk.Contains(rucksack.Right, item) {
			return item, nil
		}
	}

	return 0, errors.New("No dupliacte item found in Rucksack " + fmt.Sprint(rucksack))

}
func calculatePriority(item rune) int {
	if item >= 'a' && item <= 'z' {
		return int(item - 'a' + 1)
	} else {
		return int(item - 'A' + 27)
	}

}

func parseInput(input string) []Rucksack {
	rucksacks := []Rucksack{}
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		numberOfItems := len(line)
		rucksack := Rucksack{
			parseCompartment(line[0 : numberOfItems/2]),
			parseCompartment(line[numberOfItems/2 : numberOfItems]),
		}
		rucksacks = append(rucksacks, rucksack)

	}
	return rucksacks
}

func parseCompartment(input string) Compartment {
	return Compartment([]rune(input))
}
