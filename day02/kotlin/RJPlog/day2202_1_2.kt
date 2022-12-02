import java.io.File
import kotlin.math.*

// tag::oneliner[]
fun day02Part1Solution() = File("day2201_puzzle_input.txt").readText().split("\r\n\r\n").map {
	it.split("\r\n").map { it.toInt() }.sum()
}.max() ?: 0
//fun day01Part1Solution() = File("day2201_puzzle_input.txt").readLines().joinToString("\n").split("\n\n").map {it.split("\n").map {it.toInt()}.sum()}.sortedDescending()[0]

fun day02Part2Solution() = File("day2201_puzzle_input.txt").readText().split("\r\n\r\n").map {
	it.split("\r\n").map { it.toInt() }.sum()
}.sorted().takeLast(3).sum()
//fun day01Part2Solution() = File("day2201_puzzle_input.txt").readLines().joinToString("\n").split("\n\n").map {it.split("\n").map {it.toInt()}.sum()}.sortedDescending().take(3).sum()
// end::oneliner[]


// tag::calories[]
fun rockPaperScissors(in1: Int): Int {

	var result: Int = 0

	File("day2202_puzzle_input.txt").forEachLine {
		if (in1 == 1) {
			when (it) {
				("A X") -> result += 1 + 3
				("A Y") -> result += 2 + 6
				("A Z") -> result += 3 + 0
				("B X") -> result += 1 + 0
				("B Y") -> result += 2 + 3
				("B Z") -> result += 3 + 6
				("C X") -> result += 1 + 6
				("C Y") -> result += 2 + 0
				("C Z") -> result += 3 + 3
			}
		} else {
			when (it) {
				("A X") -> result += 3 + 0  // lose
				("A Y") -> result += 1 + 3  // draw
				("A Z") -> result += 2 + 6  // win
				("B X") -> result += 1 + 0
				("B Y") -> result += 2 + 3
				("B Z") -> result += 3 + 6
				("C X") -> result += 2 + 0
				("C Y") -> result += 3 + 3
				("C Z") -> result += 1 + 6
			}
		}
	}
	return result
}
// end::calories[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = rockPaperScissors(1)
	var solution2 = rockPaperScissors(2)

// tag::output[]
// print solution for part 1
	println("*********************************")
	println("--- Day 2: Rock Paper Scissors---")
	println("*********************************")
	println("Solution for part1")
	println("   $solution1 is your total score?")
	println()
// print solution for part 2
	println("*********************************")
	println("Solution for part2")
	println("   $solution2 would be your total score")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}