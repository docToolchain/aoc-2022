import java.io.File
import kotlin.math.*

// define data class containing all necessary data of one folder
data class Directory(val name: String) {
	var subDirectories = mutableListOf<Directory>()
	var parentDirectory: String = ""
	var sizeOfFiles: Long = 0
}

// tag::NoSpace[]
fun NoSpace(in1: Int): Long {

	// create Folderstructure
	var folderList = mutableListOf<Directory>()
	var currentFolder: String = ""

	File("day2207_puzzle_input.txt").forEachLine {
		if (it.contains("$ cd")) {
			var parameter = it.split(" ")[2]
			if (parameter != "..") {
				parameter = currentFolder + it.split(" ")[2]
				if (!folderList.contains(Directory(parameter))) {
					folderList.add(Directory(parameter))
					folderList[folderList.indexOf(Directory(parameter))].parentDirectory = currentFolder
				}
				currentFolder = parameter
			} else {
				currentFolder = folderList[folderList.indexOf(Directory(currentFolder))].parentDirectory
			}
		} else if (it[0].isDigit()) {
			folderList[folderList.indexOf(Directory(currentFolder))].sizeOfFiles += it.split(" ")[0].toLong()
		} else if (it.take(3) == "dir") {
			folderList[folderList.indexOf(Directory(currentFolder))].subDirectories.add(
				Directory(
					currentFolder + it.split(
						" "
					)[1]
				)
			)
		}
	}

	// aggregate all folder sizes
	var aggEnd = false
	var folderSize = mutableListOf<Directory>()
	folderSize.addAll(folderList)

	while (!aggEnd) {
		aggEnd = true
		folderSize.forEach {
			if (it.subDirectories.size > 0) {
				var subDirectory = it.subDirectories[0].name
				if (folderSize[folderSize.indexOf(Directory(subDirectory))].subDirectories.size == 0) {
					it.subDirectories.removeAt(0)
					it.sizeOfFiles += folderSize[folderSize.indexOf(Directory(subDirectory))].sizeOfFiles
					aggEnd = false
				}
			}
		}
	}

	// use previous calculations for both part1 and part2
	if (in1 == 1) {
		// sum up all folders with almost 100000
		var result: Long = 0
		folderSize.forEach {
			if (it.sizeOfFiles <= 100000) {
				result += it.sizeOfFiles
			}
		}
		return result
	} else {
		// search for the total disk space
		var totalDiskSpace = folderSize.map { it.sizeOfFiles }.max() ?: 0

		// calculate the value which needs to be freed up
		var amountToFreeUp = totalDiskSpace - 40000000

		// return the smallest folder size which is still higher than the space to be freed up
		return folderSize.map { it.sizeOfFiles }.filter { (it - amountToFreeUp) > 0 }.min() ?: 0
	}
}
// end::NoSpace[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = NoSpace(1)
	var solution2 = NoSpace(2)

// tag::output[]
// print solution for part 1
	println("**************************************")
	println("--- Day 7: No Space Left On Device ---")
	println("**************************************")
	println("Solution for part1")
	println("   $solution1 is the sum of the total sizes of those directories") 
	println()
// print solution for part 2
	println("**************************************")
	println("Solution for part2")
	println("   $solution2 is the total size of that directory")
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
