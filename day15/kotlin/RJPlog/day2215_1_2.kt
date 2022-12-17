import java.io.File
import kotlin.math.*

// tag::part1[]
fun BeaconExclusion(in1: Int): Int {
	var sensorList = mutableListOf<Pair<Int, Int>>()
	var beaconList = mutableListOf<Pair<Int, Int>>()
	var resultList = mutableListOf<Pair<Int, Int>>()

	File("day2215_puzzle_input.txt").forEachLine {
		var xSensor = it.split(":")[0].substringAfter("x=").substringBefore(", y=").toInt()
		var ySensor = it.split(":")[0].substringAfter("y=").toInt()
		var xBeacon = it.split(":")[1].substringAfter("x=").substringBefore(", y=").toInt()
		var yBeacon = it.split(":")[1].substringAfter("y=").toInt()
		
		sensorList.add(Pair(xSensor, ySensor))
		beaconList.add(Pair(xBeacon, yBeacon))

		var manhatten = abs(xSensor - xBeacon) + abs(ySensor - yBeacon)
		var yRange = manhatten - abs(ySensor - in1)

		if (yRange >= 0) {
			resultList.add(Pair(xSensor, in1))
			for (i in 1..yRange) {
				resultList.add(Pair(xSensor - i, in1))
				resultList.add(Pair(xSensor + i, in1))
			}
		}
	}
	return ((resultList.distinct() - sensorList) - beaconList).count()
}
// end::part1[]

// tag::part2[]
fun FindBeacon(): Long {
	var sensorList = mutableListOf<Pair<Int, Int>>()
	var beaconList = mutableListOf<Pair<Int, Int>>()

	File("day2215_puzzle_input.txt").forEachLine {
		var xSensor = it.split(":")[0].substringAfter("x=").substringBefore(", y=").toInt()
		var ySensor = it.split(":")[0].substringAfter("y=").toInt()
		var xBeacon = it.split(":")[1].substringAfter("x=").substringBefore(", y=").toInt()
		var yBeacon = it.split(":")[1].substringAfter("y=").toInt()
		sensorList.add(Pair(xSensor, ySensor))
		beaconList.add(Pair(xBeacon, yBeacon))
	}

	// find all pairs of inputs where still one line is left between both spotted areas (in this line a undiscovered beacon could hide
	var candidates = mutableListOf<Int>()
	for (i in 0..sensorList.size - 1) {
		for (j in 0..sensorList.size - 1) {
			if (i < j) {
				var manhatten1 =
					abs(sensorList[i].first - beaconList[i].first) + abs(sensorList[i].second - beaconList[i].second)
				var manhatten2 =
					abs(sensorList[j].first - beaconList[j].first) + abs(sensorList[j].second - beaconList[j].second)
				var manhatten3 =
					abs(sensorList[i].first - sensorList[j].first) + abs(sensorList[i].second - sensorList[j].second)

				if (manhatten3 == manhatten1 + manhatten2 + 2) {
					candidates.add(i)
					candidates.add(j)
				}
			}
		}
	}

	// this code does only work if there are only two pairs of input were found, means that there are two lines where a beacon could hide, the position
	// to find lies at the intersection of both lines
	var man1 =
		abs(sensorList[candidates[0]].first - beaconList[candidates[0]].first) + abs(sensorList[candidates[0]].second - beaconList[candidates[0]].second)
	var man2 =
		abs(sensorList[candidates[1]].first - beaconList[candidates[1]].first) + abs(sensorList[candidates[1]].second - beaconList[candidates[1]].second)
	var man3 =
		abs(sensorList[candidates[2]].first - beaconList[candidates[2]].first) + abs(sensorList[candidates[2]].second - beaconList[candidates[2]].second)
	var man4 =
		abs(sensorList[candidates[3]].first - beaconList[candidates[3]].first) + abs(sensorList[candidates[3]].second - beaconList[candidates[3]].second)

	var xStart = 0

	if (sensorList[candidates[0]].first < sensorList[candidates[1]].first) {
		xStart = sensorList[candidates[0]].first + man1 + 1
	} else {
		xStart = sensorList[candidates[0]].first - man1 - 1
	}

	var yStart = sensorList[candidates[0]].second

	var direction = 0
	if (sensorList[candidates[0]].second < sensorList[candidates[1]].second) {
		direction = 1
	} else {
		direction = -1
	}

	var xResult = 0
	var yResult = 0

	var gameEnd = false

	while (!gameEnd) {
		xStart -= 1
		yStart += direction
		if (abs(xStart - sensorList[candidates[2]].first) + abs(yStart - sensorList[candidates[2]].second) > man3 && abs(
				xStart - sensorList[candidates[3]].first
			) + abs(yStart - sensorList[candidates[3]].second) > man4
		) {
			xResult = xStart
			yResult = yStart
			gameEnd = true
		}
	}

	return xResult * 4000000L + yResult
}
// end::part2[]

fun main() {

	var t1 = System.currentTimeMillis()

	var solution1 = BeaconExclusion(2000000)
	var solution2 = FindBeacon()

// tag::output[]
// print solution for part 1
	println("*************************************")
	println("--- Day 15: Beacon Exclusion Zone ---")
	println("*************************************")
	println("Solution for part1")
	println("   $solution1 positions cannot contain a beacon")
	println()
// print solution for part 2
	println("*************************************")
	println("Solution for part2")
	println("   $solution2 is its tuning frequency")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
