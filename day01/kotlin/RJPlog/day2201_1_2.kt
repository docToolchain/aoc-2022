import java.io.File
import kotlin.math.*


// tag::calories[]
fun calories(in1: Int): Int {
	
	var caloriesPerElf = mutableListOf<Int>()
	var calories: Int = 0
	
	File("day2201_puzzle_input.txt").forEachLine {
		if (it == "") {
			caloriesPerElf.add(calories)
			calories = 0
		} else {
			calories += it.toInt()
		}
	}
	caloriesPerElf.add(calories)

	if (in1 == 1) {
		return caloriesPerElf.max() ?: 0
	} else {
		return caloriesPerElf.sorted().takeLast(3).sum()
	}

}
// end::calories[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = calories(1)
	var solution2 = calories(2)

// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Day 1: Calorie Counting ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is that Elf carrying?")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 are those Elves carrying")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}