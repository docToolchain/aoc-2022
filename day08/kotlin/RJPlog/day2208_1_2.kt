import java.io.File
import kotlin.math.*

// tag::treeIsVisible[]
fun treeIsVisible(x: Int, y: Int, treeList: String, width: Int, height: Int): Boolean {
	var isVisibleLeft: Boolean = true
	var isVisibleRight: Boolean = true
	var isVisibleUp: Boolean = true
	var isVisibleDown: Boolean = true

	var currentTree = treeList[x + y * width]

	for (i in 0..x - 1) {
		if (treeList[i + y * width] >= currentTree) isVisibleLeft = false
	}
	for (i in x + 1..width - 1) {
		if (treeList[i + y * width] >= currentTree) isVisibleRight = false
	}
	for (i in 0..y - 1) {
		if (treeList[x + i * width] >= currentTree) isVisibleUp = false
	}
	for (i in y + 1..height - 1) {
		if (treeList[x + i * width] >= currentTree) isVisibleDown = false
	}
	return isVisibleLeft || isVisibleRight || isVisibleUp || isVisibleDown
}
// end::treeIsVisible[]

// tag::senicScore[]
fun senicScore(x: Int, y: Int, treeList: String, width: Int, height: Int): Int {
	var senScoLeft: Int = 0
	var senScoRight: Int = 0
	var senScoUp: Int = 0
	var senScoDown: Int = 0

	var currentTree = treeList[x + y * width]

	var equalTreeReached = false

	for (i in x - 1 downTo 0) {
		if (!equalTreeReached) {
			if (treeList[i + y * width] < currentTree) {
				senScoLeft += 1
			} else {
				senScoLeft += 1
				equalTreeReached = true
			}
		}
	}
	equalTreeReached = false

	for (i in x + 1..width - 1) {
		if (!equalTreeReached) {
			if (treeList[i + y * width] < currentTree) {
				senScoRight += 1
			} else {
				senScoRight += 1
				equalTreeReached = true
			}
		}
	}
	equalTreeReached = false

	for (i in y - 1 downTo 0) {
		if (!equalTreeReached) {
			if (treeList[x + i * width] < currentTree) {
				senScoUp += 1
			} else {
				senScoUp += 1
				equalTreeReached = true
			}
		}
	}
	equalTreeReached = false

	for (i in y + 1..height - 1) {
		if (!equalTreeReached) {
			if (treeList[x + i * width] < currentTree) {
				senScoDown += 1
			} else {
				senScoDown += 1
				equalTreeReached = true
			}
		}
	}

	return senScoLeft * senScoRight * senScoUp * senScoDown
}
// end::senicScore[]

// tag::TreeTreeHouse[]
fun TreeTreeHouse(in1: Int): Int {
	var width = 0
	var height = 0
	var treeList: String = ""

	//store input in a single string, single trees are accesible by their coordinates (index = x + y*width) 
	File("day2208_puzzle_input.txt").forEachLine {
		width = it.length
		height += 1
		treeList += it
	}

	// initialize result with outer lines
	var resultPart1: Int = (height + width) * 2 - 4
	var resultPart2: Int = 0

	// start for each puzzle part the calculation for each tree
	for (y in 1..height - 2) {
		for (x in 1..width - 2) {
			if (in1 == 1) {
				if (treeIsVisible(x, y, treeList, width, height)) {
					resultPart1 += 1
				}
			} else {
				var senSco = senicScore(x, y, treeList, width, height)
				if (senSco > resultPart2) resultPart2 = senSco
			}
		}
	}

	if (in1 == 1) {
		return resultPart1
	} else {
		return resultPart2
	}
}
// end::TreeTreeHouse[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = TreeTreeHouse(1)
	var solution2 = TreeTreeHouse(2)

// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Treetop Tree House ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 trees are visible from outside the grid")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the highest scenic score possible for any tree")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}