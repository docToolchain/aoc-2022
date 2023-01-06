import java.io.File
import kotlin.math.*

// tag::day2221_1[]
fun day2221_1(): String {

	var monkeyMap = mutableMapOf<String, String>()
	File("day2221_puzzle_input.txt").forEachLine {
		var instruction = it.split(": ")
		monkeyMap.put(instruction[0], instruction[1])
	}

	// replace all placeholders by numbers and calculat formulas until key root contains only a number
	while (monkeyMap.getValue("root").contains(" ")) {
		for ((key, value) in monkeyMap) {
			if (value.contains(" ")) {
				var instruction = value.split(" ")
				if (instruction[0][0].isDigit() && instruction[2][0].isDigit()) {
					when (instruction[1]) {
						"+" -> monkeyMap.put(key, (instruction[0].toLong() + instruction[2].toLong()).toString())
						"-" -> monkeyMap.put(key, (instruction[0].toLong() - instruction[2].toLong()).toString())
						"*" -> monkeyMap.put(key, (instruction[0].toLong() * instruction[2].toLong()).toString())
						"/" -> monkeyMap.put(key, (instruction[0].toLong() / instruction[2].toLong()).toString())
					}
				} else if (!instruction[0][0].isDigit()) {
					if (!monkeyMap.getValue(instruction[0]).contains(" ")) {
						monkeyMap.put(
							key,
							monkeyMap.getValue(instruction[0]) + " " + instruction[1] + " " + instruction[2]
						)
					}
				} else if (!instruction[2][0].isDigit()) {
					if (!monkeyMap.getValue(instruction[2]).contains(" ")) {
						monkeyMap.put(
							key,
							instruction[0] + " " + instruction[1] + " " + monkeyMap.getValue(instruction[2])
						)
					}
				}
			}
		}
	}
	return monkeyMap.getValue("root")
}
// end::day2221_1[]

// tag::day2221_2[]
fun day2221_2(): String {

	var monkeyMap = mutableMapOf<String, String>()

	File("day2221_puzzle_input.txt").forEachLine {
		var instruction = it.split(": ")
		if (instruction[0] == "root") {
			var operation = instruction[1].split(" ")
			monkeyMap.put(instruction[0], operation[0] + " = " + operation[2])
		} else if (instruction[0] == "humn") {
			monkeyMap.put(instruction[0], "humn")
		} else {
			monkeyMap.put(instruction[0], instruction[1])
		}

	}

	// try do resolve as much as possible of the formulars analog to part 1
	for (i in 0..100) {  // for shure this is not a good solution, but today I had not time to do some additional work in AoC :-)
		for ((key, value) in monkeyMap) {
			if (value.contains(" ")) {
				var instruction = value.split(" ")
				if (instruction[0][0].isDigit() && instruction[2][0].isDigit()) {
					when (instruction[1]) {
						"+" -> monkeyMap.put(key, (instruction[0].toLong() + instruction[2].toLong()).toString())
						"-" -> monkeyMap.put(key, (instruction[0].toLong() - instruction[2].toLong()).toString())
						"*" -> monkeyMap.put(key, (instruction[0].toLong() * instruction[2].toLong()).toString())
						"/" -> monkeyMap.put(key, (instruction[0].toLong() / instruction[2].toLong()).toString())
					}
				} else {
					if (!instruction[0][0].isDigit()) {
						if (!monkeyMap.getValue(instruction[0]).contains(" ")) {
							monkeyMap.put(
								key,
								monkeyMap.getValue(instruction[0]) + " " + instruction[1] + " " + instruction[2]
							)
						}
					}
					if (!instruction[2][0].isDigit()) {
						if (!monkeyMap.getValue(instruction[2]).contains(" ")) {
							monkeyMap.put(
								key,
								instruction[0] + " " + instruction[1] + " " + monkeyMap.getValue(instruction[2])
							)
						}
					}
				}
			}
		}
	}
    
	// calculate step by step the remaining formulas until only open placeholder is "humn"
	while (!monkeyMap.getValue("root").contains("humn")) {
		var instruction = monkeyMap.getValue("root").split(" = ")
		if (!instruction[0][0].isDigit()) {
			var operation = monkeyMap.getValue(instruction[0]).split(" ")
			if (!operation[0][0].isDigit()) {
				when (operation[1]) {
					"+" -> monkeyMap.put(
						"root",
						operation[0] + " = " + (instruction[1].toLong() - operation[2].toLong()).toString()
					)
					"-" -> monkeyMap.put(
						"root",
						operation[0] + " = " + (instruction[1].toLong() + operation[2].toLong()).toString()
					)
					"*" -> monkeyMap.put(
						"root",
						operation[0] + " = " + (instruction[1].toLong() / operation[2].toLong()).toString()
					)
					"/" -> monkeyMap.put(
						"root",
						operation[0] + " = " + (instruction[1].toLong() * operation[2].toLong()).toString()
					)
				}
			} else if (!operation[2][0].isDigit()) {
				when (operation[1]) {
					"+" -> monkeyMap.put(
						"root",
						operation[2] + " = " + (instruction[1].toLong() - operation[0].toLong()).toString()
					)
					"-" -> monkeyMap.put(
						"root",
						operation[2] + " = " + (operation[0].toLong() - instruction[1].toLong()).toString()
					)
					"*" -> monkeyMap.put(
						"root",
						operation[2] + " = " + (instruction[1].toLong() / operation[0].toLong()).toString()
					)
					"/" -> monkeyMap.put(
						"root",
						operation[2] + " = " + (operation[0].toLong() / instruction[1].toLong()).toString()
					)
				}
			}
			monkeyMap.remove(instruction[0])
		}  // else if (!instuction[2][0].isDigit())  --> not need for my puzzle input, would have been copy and apdapt of the upper part 
	}
	return monkeyMap.getValue("root")
}
// end::day2221_2[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = day2221_1()
	var solution2 = day2221_2()

// tag::output[]
// print solution for part 1
	println("***************************")
	println("--- Day 21: Monkey Math ---")
	println("***************************")
	println("Solution for part1")
	println("   $solution1 will the monkey named root yell")
	println()
// print solution for part 2
	println("***************************")
	println("Solution for part2")
	println("   $solution2 do you yell to pass root's equality test")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}