import java.io.File
import kotlin.math.*

// tag::follow[]
fun follow(xH: Int, yH: Int, xT: Int, yT: Int): Pair<Int, Int> {
	var xTNew: Int = xT
	var yTNew: Int = yT

	if ((xT != xH) && (yT != yH) && (abs(xT - xH) > 1 || abs(yT - yH) > 1)) {
		if ((xH - xT) > 0) {
			xTNew = xT + 1
		} else if ((xH - xT) < 0) {
			xTNew = xT - 1
		}
		if ((yH - yT) > 0) {
			yTNew = yT + 1
		} else if ((yH - yT) < 0) {
			yTNew = yT - 1
		}
	} else if ((xT != xH) || (yT != yH)) {
		if ((xH - xT) > 1) {
			xTNew = xT + 1
		} else if ((xH - xT) < -1) {
			xTNew = xT - 1
		}
		if ((yH - yT) > 1) {
			yTNew = yT + 1
		} else if ((yH - yT) < -1) {
			yTNew = yT - 1
		}
	}
	return Pair(xTNew, yTNew)
}
// end::follow[]

// tag::ropeBridge[]
fun RopeBridge(in1: Int): Int {

	var (xH, yH) = Pair(0, 0)
	var (xT, yT) = Pair(0, 0)
	var allPath = MutableList(10) { mutableListOf<Pair<Int, Int>>(Pair(0, 0)) }

	File("day2209_puzzle_input.txt").forEachLine {
		var (dir, steps) = it.split(" ")

		for (i in 1..steps.toInt()) {
			xH = allPath[0].takeLast(1)[0].first
			yH = allPath[0].takeLast(1)[0].second
			if (dir == "U") {
				yH += 1
			} else if (dir == "D") {
				yH -= 1
			} else if (dir == "R") {
				xH += 1
			} else if (dir == "L") {
				xH -= 1
			}
			allPath[0].add(Pair(xH, yH))

			// add loop for all 9 sections of rope
			for (j in 1..9) {
				xT = allPath[j].takeLast(1)[0].first
				yT = allPath[j].takeLast(1)[0].second
				xH = allPath[j - 1].takeLast(1)[0].first
				yH = allPath[j - 1].takeLast(1)[0].second

				val newPos = follow(xH, yH, xT, yT)

				xT = newPos.first
				yT = newPos.second
				allPath[j].add(Pair(xT, yT))
			}
		}
	}
	if (in1 == 1) {
		return allPath[1].distinct().size
	} else {
		return allPath[9].distinct().size
	}
}
// end::ropeBridge[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = RopeBridge(1)
	var solution2 = RopeBridge(2)

// tag::output[]
// print solution for part 1
	println("***************************")
	println("--- Day 9: Rope Bridge ---")
	println("**************************")
	println("Solution for part1")
	println("   $solution1 positions does the tail of the rope visit at least once")
	println()
// print solution for part 2
	println("**************************")
	println("Solution for part2")
	println("   $solution2 positions does the tail of the rope visit at least once")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}