import java.io.File
import kotlin.math.*

// tag::CheckWall[]
fun checkTilesClay(
	x: Int,
	y: Int,
	xRange: MutableList<Pair<Int, IntRange>>,
	yRange: MutableList<Pair<Int, IntRange>>
): Boolean {
	xRange.forEach {
		if (it.first == x && (it.second).contains(y)) {
			return true
		}
	}
	yRange.forEach {
		if (it.first == y && it.second.contains(x)) {
			return true
		}
	}
	return false
}
// end::CheckWall[]

fun Reservoir(in1: Int): Int {

// tag::PuzzleInputProzessing[]	
	var xRange = mutableListOf<Pair<Int, IntRange>>()
	var yRange = mutableListOf<Pair<Int, IntRange>>()
	var yMax: Int = 0

	File("day2214_puzzle_input.txt").forEachLine {
		var instruction = it.split(" -> ")
		for (i in 0..instruction.size - 2) {
			var firstRange = instruction.toList()[i].split(",")
			var secondRange = instruction[i + 1].split(",")

			if (firstRange[0] == secondRange[0]) {
				if (firstRange[1].toString().toInt() < secondRange[1].toString().toInt()) {
					xRange.add(
						Pair(
							firstRange[0].toString().toInt(),
							IntRange(firstRange[1].toString().toInt(), secondRange[1].toString().toInt())
						)
					)
					yMax = max(yMax, secondRange[1].toString().toInt())
				} else {
					xRange.add(
						Pair(
							firstRange[0].toString().toInt(),
							IntRange(secondRange[1].toString().toInt(), firstRange[1].toString().toInt())
						)
					)
					yMax = max(yMax, firstRange[1].toString().toInt())
				}
			} else {
				if (firstRange[0].toString().toInt() < secondRange[0].toString().toInt()) {
					yRange.add(
						Pair(
							firstRange[1].toString().toInt(),
							IntRange(firstRange[0].toString().toInt(), secondRange[0].toString().toInt())
						)
					)
				} else {
					yRange.add(
						Pair(
							firstRange[1].toString().toInt(),
							IntRange(secondRange[0].toString().toInt(), firstRange[0].toString().toInt())
						)
					)
				}
				yMax = max(yMax, firstRange[1].toString().toInt())
			}
		}
	}

	if (in1 == 2) {
		yRange.add(Pair(yMax + 2, IntRange(500 - yMax * 2, 500 + yMax * 2)))
	}
// end::PuzzleInputProzessing[]	

// tag::Reservoir[]
	var fallingSand = mutableListOf<Pair<Int, Int>>()
	var gameEnd: Boolean = false
	var sandCount = 0

	while (!gameEnd) {
		var x = 500
		var y = 0

		while (true) {
			// if field below is empty, go one step deper /if not go left or right / or place sand
			if (!(fallingSand.contains(Pair(x, y + 1)) || checkTilesClay(x, y + 1, xRange, yRange))) {
				y += 1
				if (in1 == 1) {
					if (y > yMax) {
						break
					}
				}
			} else if (!(fallingSand.contains(Pair(x - 1, y + 1)) || checkTilesClay(x - 1, y + 1, xRange, yRange))) {
				x -= 1
				y += 1
			} else if (!(fallingSand.contains(Pair(x + 1, y + 1)) || checkTilesClay(x + 1, y + 1, xRange, yRange))) {
				x += 1
				y += 1
			} else {
				fallingSand.add(Pair(x, y))
				break
			}
		}
		if (in1 == 2) {
			if (fallingSand.contains(Pair(500, 0))) {
				gameEnd = true
			}
		} else {
			if (sandCount == fallingSand.count()) {
				gameEnd = true
			}
		}
		sandCount = fallingSand.count()
	}
	return sandCount
// end::Reservoir[]
}


fun main() {

	var t1 = System.currentTimeMillis()

	var solution1 = Reservoir(1)
	var solution2 = Reservoir(2)

// tag::output[]
// print solution for part 1
	println("**********************************")
	println("--- Day 14: Regolith Reservoir ---")
	println("**********************************")
	println("Solution for part1")
	println("   $solution1 units of sand come to rest")
	println()
// print solution for part 2
	println("**********************************")
	println("Solution for part2")
	println("   $solution2 units of sand come to rest")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}

