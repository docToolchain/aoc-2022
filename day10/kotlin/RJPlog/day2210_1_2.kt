import java.io.File
import kotlin.math.*

// tag::CathodRay[]
fun CathodRay(in1: Int): Int {

	var signalStrength = mutableListOf<Int>()
	var messurePoint = mutableListOf(20, 60, 100, 140, 180, 220)
	var output = mutableListOf<Int>()
	var cycle: Int = 1
	var register: Int = 1
	output.add(register)

	File("day2210_puzzle_input.txt").forEachLine {
		if (it == "noop") {
			cycle += 1
			output.add(register)
			if (messurePoint.contains(cycle)) {
				signalStrength.add(cycle * register)
			}

		} else if (it.contains("addx")) {
			var value = it.substringAfter("addx ").toInt()
			cycle += 1
			output.add(register)
			if (messurePoint.contains(cycle)) {
				signalStrength.add(cycle * register)
			}
			cycle += 1
			register += value
			output.add(register)
			if (messurePoint.contains(cycle)) {
				signalStrength.add(cycle * register)
			}

		}
	}
	// return value for different Parts
	if (in1 == 1) {
		return signalStrength.sum()
	} else {
		for (y in 0..5) {
			for (x in 0..39) {
				var sprite = output[x + y * 40]
				if (x == sprite || x == sprite - 1 || x == sprite + 1) {
					print("#")
				} else {
					print(".")
				}
			}
			println()
		}
	}
	return -1
}
// end::CathodRay[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = CathodRay(1)


// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Day 10: Cathode-Ray Tube ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is the sum of these six signal strengths.")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	var solution2 = CathodRay(2)
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}