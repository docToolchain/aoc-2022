import java.io.File
import kotlin.math.*

// tag::SupplyStack[]
fun SuSa(in1: Int): String {

	var stackStaples = mutableListOf<String>()

	stackStaples.add("TVJWNRMS")
	stackStaples.add("VCPQJDWB")
	stackStaples.add("PRDHFJB")
	stackStaples.add("DNMBPRF")
	stackStaples.add("BTPRVH")
	stackStaples.add("TPBC")
	stackStaples.add("LPRJB")
	stackStaples.add("WBZTLSCN")
	stackStaples.add("GSL")

	File("day2205_puzzle_input.txt").forEachLine {

		if (it.contains("move")) {
			var num = it.substringAfter("move ").substringBefore(" from").toInt()
			var stFrom = it.substringAfter("from ").substringBefore(" to").toInt()
			var stTo = it.substringAfter("to ").toInt()

			if (in1 == 1) {
				for (i in 1..num) {
					var newTo = stackStaples[stFrom - 1].take(1) + stackStaples[stTo - 1]
					var newFrom = stackStaples[stFrom - 1].drop(1)
					stackStaples.removeAt(stFrom - 1)
					stackStaples.add(stFrom - 1, newFrom)
					stackStaples.removeAt(stTo - 1)
					stackStaples.add(stTo - 1, newTo)
				}
			} else {
					var newTo = stackStaples[stFrom - 1].take(num) + stackStaples[stTo - 1]
					var newFrom = stackStaples[stFrom - 1].drop(num)
					stackStaples.removeAt(stFrom - 1)
					stackStaples.add(stFrom - 1, newFrom)
					stackStaples.removeAt(stTo - 1)
					stackStaples.add(stTo - 1, newTo)
			}
		}

	}

	var result: String = ""
	stackStaples.forEach {
		result = result + it.take(1)
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