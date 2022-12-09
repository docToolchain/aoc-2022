package main

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strconv"
	"strings"

	"github.com/thoas/go-funk"
)

func main() {
	log.Println("Hello, World!")
	a()
}

func a() {
	test := `1000
2000
3000

4000

5000
6000

7000
8000
9000

10000`
	testElves := parseInput(test)
	fmt.Println(testElves)
	puzzleInput, _ := ioutil.ReadFile("./input.txt")
	elves := parseInput(string(puzzleInput))
	fmt.Println(elves)
	bags := []int{}
	for _, elve := range elves {
		calories := 0
		for _, item := range elve {
			calories = calories + item
		}
		bags = append(bags, calories)
	}

	sort.Ints(bags)
	funk.ReverseInt(bags)

	maxCalories := bags[:3]
	fmt.Println(maxCalories)
	fmt.Println(funk.Reduce(maxCalories, '+', 0))

}

func parseInput(input string) [][]int {
	elves := [][]int{}
	elveCounter := 0
	scanner := bufio.NewScanner(strings.NewReader(input))
	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			if len(elves) <= elveCounter {
				elves = append(elves, []int{})
			}
			if i, err := strconv.Atoi(line); err == nil {
				elves[elveCounter] = append(elves[elveCounter], i)
			}
		} else {
			elveCounter++
		}
	}
	return elves
}
