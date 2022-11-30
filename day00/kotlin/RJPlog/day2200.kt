import java.io.File
import kotlin.math.*

// tag::read_input[]
fun read(): List<String> {
	var result = mutableListOf<String>()
	File("day2200_puzzle_input.txt").forEachLine {
		result.add(it)
	}
	return result
}
// end::read_input[]


fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = read()[0]
	var solution2 = read()[1]

// tag::output[]
// print solution for part 1
	println("*****************************")
	println("--- Day 00: Hello World ---")
	println("*****************************")
	println("Solution for part1")
	println("   $solution1")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}