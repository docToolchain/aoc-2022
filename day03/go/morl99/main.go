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
	b()
}

type Group [3]Rucksack

type Rucksack []rune

func b() {
	puzzleInput, _ := ioutil.ReadFile("./input.txt")
	groups := parseInput(string(puzzleInput))
	sum := 0
	for _, group := range groups {
		badge, error := findBadge(group)
		if error != nil {
			fmt.Println(error)
		} else {
			sum += calculatePriority(badge)
		}
	}
	fmt.Println(sum)
}

func findBadge(group Group) (rune, error) {
	for _, item := range group[0] {
		if funk.Contains(group[1], item) && funk.Contains(group[2], item) {
			return item, nil
		}
	}
	return '0', errors.New("Found no badge in group " + fmt.Sprint(group))
}

func calculatePriority(item rune) int {
	if item >= 'a' && item <= 'z' {
		return int(item - 'a' + 1)
	} else {
		return int(item - 'A' + 27)
	}

}

func parseInput(input string) []Group {
	groups := []Group{}
	scanner := bufio.NewScanner(strings.NewReader(input))
	lineNumber := 0
	var nextGroup Group
	for scanner.Scan() {
		line := scanner.Text()
		nextGroup[lineNumber%3] = parseRucksack(line)
		lineNumber++
		if lineNumber%3 == 0 {
			groups = append(groups, nextGroup)
		}
	}
	return groups
}

func parseRucksack(input string) Rucksack {
	return []rune(input)
}
