import java.io.File
import kotlin.math.*

// tag::CampCleanUp[]
fun CampCleanUp(in1: Int): Int {

	var result: Int = 0

	File("day2204_puzzle_input.txt").forEachLine {
		var (a, b, c, d) = """(\d+)-(\d+),(\d+)-(\d+)""".toRegex().find(it)!!.destructured
		var elf1 = IntRange(a.toInt(),b.toInt())
		var elf2 = IntRange(c.toInt(),d.toInt())
		if (in1 == 1) {
			if ( (elf1.contains(c.toInt()) && elf1.contains(d.toInt())) || (elf2.contains(a.toInt()) && elf2.contains(b.toInt()))) {
				result += 1
			}
		} else {
			if (elf1.contains(c.toInt()) || elf1.contains(d.toInt()) || elf2.contains(a.toInt()) || elf2.contains(b.toInt()) ) {
				result += 1
			}
		}
	}
	return result
}
// end::CampCleanUp[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = CampCleanUp(1)
	var solution2 = CampCleanUp(2)

// tag::output[]
// print solution for part 1
	println("***************************")
	println("--- Day 4: Camp Cleanup ---")
	println("***************************")
	println("Solution for part1")
	println("   $solution1 is that assignment pairs does one range fully contain the other.")
	println()
// print solution for part 2
	println("***************************")
	println("Solution for part2")
	println("   $solution2 assignment pairs do the ranges overlap")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}