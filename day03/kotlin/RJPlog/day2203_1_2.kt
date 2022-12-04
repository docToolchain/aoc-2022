import java.io.File
import kotlin.math.*

// tag::RuReAlternatives[]
fun RuReOneAlt1() = File("day2203_puzzle_input.txt").readLines()
	.map { (it.chunked(it.length / 2)[0].toList() - (it.chunked(it.length / 2)[0].toList() - it.chunked(it.length / 2)[1].toList())).joinToString() }
	.map { if (it.toCharArray()[0].toInt() > 95) it.toCharArray()[0].toInt() - 96 else it.toCharArray()[0].toInt() - 64 + 26 }
	.sum()

fun RuReOneAlt2() = File("day2203_puzzle_input.txt").readLines()
	.chunked(3)
	.map {it[0].toList() - (it[0].toList()-it[1].toList()) - (it[1].toList()-it[2].toList())}
	.map { if (it.toCharArray()[0].toInt() > 95) it.toCharArray()[0].toInt() - 96 else it.toCharArray()[0].toInt() - 64 + 26 }
	.sum()
// end::RuReAlternatives[]

// tag::RuRePartOne[]
fun RuRePartOne(): Int {

	var result: Int = 0

	File("day2203_puzzle_input.txt").forEachLine {
		val compOne = it.chunked(it.length / 2)[0]
		val compTwo = it.chunked(it.length / 2)[1]
		for (i in 1..52) {
			var item: Char
			if (i < 27) {
				item = (i + 96).toChar()
			} else {
				item = (i + 64 - 26).toChar()
			}
			if (compOne.contains(item) && compTwo.contains(item)) {
				result += i
				break
			}
		}
	}
	return result
}
// end::RuRePartOne[]

// tag::RuRePartTwo[]
fun RuRePartTwo(): Int {

	var result: Int = 0
	var groupCount: Int = 0
	var compOne = ""
	var compTwo = ""
	var compThree = ""

	File("day2203_puzzle_input.txt").forEachLine {
		if (groupCount == 0) {
			compOne = it
		} else if (groupCount == 1) {
			compTwo = it
		} else {
			compThree = it
			for (i in 1..52) {
				var item: Char
				if (i < 27) {
					item = (i + 96).toChar()
				} else {
					item = (i + 64 - 26).toChar()
				}
				if (compOne.contains(item) && compTwo.contains(item) && compThree.contains(item)) {
					result += i
					break
				}
			}
			groupCount = -1
		}
		groupCount += 1
	}
	return result
}
// end::RuRePartTwo[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = RuRePartOne()
	var solution2 = RuRePartTwo()

// tag::output[]
// print solution for part 1
	println("**************************************")
	println("--- Day 3: Rucksack Reorganization ---")
	println("**************************************")
	println("Solution for part1")
	println("   $solution1 is the sum of the priorities of those item types")
	print("   alternative: ")
	println(RuReOneAlt1())
	println()
// print solution for part 2
	println("**************************************")
	println("Solution for part2")
	println("   $solution2 is the sum of the priorities of those item types")
	print("   alternative: ")
	println(RuReOneAlt2())
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
