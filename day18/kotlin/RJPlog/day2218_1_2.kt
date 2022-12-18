import java.io.File
import kotlin.math.*

// tag::day2218[]
fun BoiBou(in1: Int): Int {

	var bouldersList = mutableListOf<Triple<Int, Int, Int>>()
	var result = 0

	var xMin = 0
	var xMax = 0
	var yMin = 0
	var yMax = 0
	var zMin = 0
	var zMax = 0

	File("day2218_puzzle_input.txt").forEachLine {
		var (x, y, z) = it.split(",").map { it.toString().toInt() }
        
		// needed for part two
		xMin = min(xMin, x)
		xMax = max(xMax, x)
		yMin = min(yMin, y)
		yMax = max(yMax, y)
		zMin = min(zMin, z)
		zMax = max(zMax, z)
        
		// for each new cube 6 surface areas are added
		result += 6
		
		bouldersList.forEach {
			// for each already existing cube which is touched by the new cube, two surface areas are removed
			if (abs(x - it.first) + abs(y - it.second) + abs(z - it.third) == 1) {
				result -= 2
			}
		}
		bouldersList.add(Triple(x, y, z))
	}
    
	// return result for first part
	if (in1 == 1) {
		return result
	}

	// start second part - find all free cubes which are outside
	var gameEnd = false
	var freeCubeList = mutableListOf<Triple<Int, Int, Int>>()

	while (!gameEnd) {
		gameEnd = true

		for (z in zMin - 1..zMax + 1) {
			for (y in yMin - 1..yMax + 1) {
				for (x in xMin - 1..xMax + 1) {
					if (!bouldersList.contains(Triple(x, y, z))) {
						if (x == xMin || x == xMax || y == yMin || y == yMax || z == zMin || z == zMax) {
							// set cubes to the outside boundaries
							if (!freeCubeList.contains(Triple(x, y, z))) {
								freeCubeList.add(Triple(x, y, z))
							}
						} else {
							// if not outside boundary add a new free cube, if the position is connected to the outside
							var connected = false
							freeCubeList.forEach {
								if (abs(x - it.first) + abs(y - it.second) + abs(z - it.third) == 1) {
									connected = true
								}
							}
							if (connected && !freeCubeList.contains(Triple(x, y, z))) {
								freeCubeList.add(Triple(x, y, z))
								gameEnd = false
							}
						}
					}
				}
			}
		}
	}

	// count all surfaces of the free cubes connected to a boiling boulder cube
	result = 0
	freeCubeList.forEach {
		var xC = it.first
		var yC = it.second
		var zC = it.third
		bouldersList.forEach {
			if (abs(xC - it.first) + abs(yC - it.second) + abs(zC - it.third) == 1) {
				result += 1
			}
		}
	}
    
	// return result for part 2
	return result
}
// end::day2218[]


fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = BoiBou(1)
	var solution2 = BoiBou(2)

// tag::output[]
// print solution for part 1
	println("********************************")
	println("--- Day 18: Boiling Boulders ---")
	println("********************************")
	println("Solution for part1")
	println("   $solution1 is the surface area of your scanned lava droplet")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the exterior surface area of your scanned lava droplet")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}