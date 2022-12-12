import java.io.File
import kotlin.math.*

// tag::HillClimbing[]
fun HillClimbing(in1: Int): Int {

	var landscape: String = ""
	var xStart: Int = 0
	var yStart: Int = 0
	var xEnd: Int = 0
	var yEnd: Int = 0
	var width: Int = 0
	var height: Int = 0
	
	File("day2212_puzzle_input.txt").forEachLine {
		width = it.length
		if (it.contains("S")) {
			xStart = it.indexOf("S")
			yStart = height
		}
		if (it.contains("E")) {
			xEnd = it.indexOf("E")
			yEnd = height
		}
		landscape += it.replace("S", "a").replace("E", "z")
		height += 1
	}

	var distList = MutableList(width * height) { width * height }
	distList[xStart * yStart * width] = 0

	if (in1 == 2) {
		for (y in 0..height - 1) {
			for (x in 0..width - 1) {
				if (landscape[x + y * width] == 'a') {
					distList[x + y * width] = 0
				}
			}
		}
	}

	// iterate over all points
	var gameEnd: Boolean = false
	var distListSum = distList.sum()

	while (!gameEnd) {
		for (y in 0..height - 1) {
			for (x in 0..width - 1) {
				// check successors of all known nodes
				var dist = distList[x + y * width]
				if (dist != width * height) {
					// calculate all possible directions
					if (x - 1 >= 0) {
						if (landscape[(x - 1) + y * width] <= landscape[x + y * width] + 1) {
							distList[x - 1 + y * width] = min(distList[(x - 1) + y * width], dist + 1)
						}
					}
					if (x + 1 < width) {
						if (landscape[(x + 1) + y * width] <= landscape[x + y * width] + 1) {
							distList[(x + 1) + y * width] = min(distList[(x + 1) + y * width], dist + 1)
						}
					}
					if (y - 1 >= 0) {
						if (landscape[x + (y - 1) * width] <= landscape[x + y * width] + 1) {
							distList[x + (y - 1) * width] = min(distList[x + (y - 1) * width], dist + 1)
						}
					}
					if (y + 1 < height) {
						if (landscape[x + (y + 1) * width] <= landscape[x + y * width] + 1) {
							distList[x + (y + 1) * width] = min(distList[x + (y + 1) * width], dist + 1)
						}
					}
				}
			}
		}
		if (distListSum == distList.sum()) {
			gameEnd = true
		}
		distListSum = distList.sum()
	}
	return distList[xEnd + yEnd * width]
}
// end::Hillclimbing[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = HillClimbing(1)
	var solution2 = HillClimbing(2)

// tag::output[]
// print solution for part 1
	println("***************************************")
	println("--- Day 12: Hill Climbing Algorithm ---")
	println("***************************************")
	println("Solution for part1")
	println("   $solution1 is the fewest steps.")
	println()
// print solution for part 2
	println("***************************************")
	println("Solution for part2")
	println("   $solution2 is the fewest steps.")
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}