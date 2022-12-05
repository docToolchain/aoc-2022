import java.io.File
import kotlin.math.*

// tag::SupplyStack[]
fun SuSa(in1: Int): String {

	var stackStaplesNew = MutableList(20) { mutableListOf<Char>() }

	File("day2205_puzzle_input.txt").forEachLine {
		if (it.contains("[")) {
			for (i in 0..(it.length) / 4) {
				if (it[i * 4 + 1] != ' ') {
					stackStaplesNew[i].add(it[i * 4 + 1])
				}
			}
		}

		if (it.contains("move")) {
			var num = it.substringAfter("move ").substringBefore(" from").toInt()
			var stFrom = it.substringAfter("from ").substringBefore(" to").toInt()
			var stTo = it.substringAfter("to ").toInt()

			if (in1 == 1) {
				for (i in 1..num) {
					stackStaplesNew[stTo - 1].add(0, stackStaplesNew[stFrom - 1][0])
					stackStaplesNew[stFrom - 1].removeAt(0)
				}

			} else {
				for (i in 1..num) {
					stackStaplesNew[stTo - 1].add(i-1, stackStaplesNew[stFrom - 1][0])
					stackStaplesNew[stFrom - 1].removeAt(0)
				}
			}
		}
	}

	var result: String = ""
	stackStaplesNew.forEach {
		result = result + it.take(1).joinToString()
	}
	return result
}
// end::SupplyStack[]


fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = SuSa(1)
	var solution2 = SuSa(2)

// tag::output[]
// print solution for part 1
	println("****************************")
	println("--- Day 5: Supply Stacks ---")
	println("****************************")
	println("Solution for part1")
	println("   $solution1 crate ends up on top of each stack")
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