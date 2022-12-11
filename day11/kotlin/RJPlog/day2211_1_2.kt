import java.io.File
import kotlin.math.*

// tag::Monkey[]
data class Monkey(var number: Int) {
	var items = mutableListOf<Long>()
	var operation: String = ""
	var secondValue: String = ""
	var test: Long = 0
	var ifTrue: Int = 0
	var ifFalse: Int = 0
	var counts:Long = 0
}
// end::Monkey[]

// tag::MonkeyMiddle[]
fun MonkeyMiddle(in1: Int): Long {

	// parse puzzle input and fill list of monkeys with their roles and contents
	var monkeyList = mutableListOf<Monkey>()
	var monkey: Int = 0
	var primeFactor = 1
	
	File("day2211_puzzle_input.txt").forEachLine {
		if (it.contains("Monkey")) {
			monkey = it.substringAfter("Monkey ").dropLast(1).toInt()
			monkeyList.add(Monkey(monkey))
		} else if (it.contains("Starting")) {
			var items = it.substringAfter("Starting items: ").split(", ")
			items.forEach {
				monkeyList[monkey].items.add(it.toLong())
			}
		} else if (it.contains("Operation")) {
			var items = it.substringAfter("Operation: new = ").split(" ")
			monkeyList[monkey].operation = items[1]
			monkeyList[monkey].secondValue = items[2]
		} else if (it.contains("Test")) {
			var items = it.substringAfter("Test: divisible by ")
			monkeyList[monkey].test = items.toLong()
			primeFactor *= items.toInt()
		} else if (it.contains("true")) {
			var items = it.substringAfter("If true: throw to monkey ")
			monkeyList[monkey].ifTrue = items.toInt()
		} else if (it.contains("false")) {
			var items = it.substringAfter("If false: throw to monkey ")
			monkeyList[monkey].ifFalse = items.toInt()
		}
	}
	
	// controll data flow for part one and part two
	var n : Int
	if (in1 == 1) {
		n = 20
	} else {
		n = 10000
	} 
    
	// play turns of the monkey in the middle game
	for (i in 1..n) {
		monkeyList.forEach {
			var operation = it.operation
			var secondValue = it.secondValue
			var test = it.test
			var ifTrue = it.ifTrue
			var ifFalse = it.ifFalse
			it.items.forEach {			
				var worry: Long = 0
				var b: Long 
				if (secondValue == "old") {
						b = it
				} else {
						b = secondValue.toLong()
				}
				if (operation == "+") {
						worry = it + b
				} else if (operation == "-") {
						worry = it - b
				} else if (operation == "*") {
						worry = it * b
				}
				if( in1 == 3) {
				worry = worry / 3
				} else {
					worry = worry % (primeFactor)
				}		
				if (worry % test == 0L) {
					monkeyList[ifTrue].items.add(worry)
				} else {
					monkeyList[ifFalse].items.add(worry)
				}
			}
			it.counts += it.items.size
			it.items.clear()
		}
	}
	
	// prepare result
	var countsList = mutableListOf<Long>()
	monkeyList.forEach{
		countsList.add(it.counts)
	}
	countsList.sortDescending()

	return countsList[0]*countsList[1]
}
// end::MonkeyMiddle[]

fun main() {
	var t1 = System.currentTimeMillis()

	var solution1 = MonkeyMiddle(1)
	var solution2 = MonkeyMiddle(2)

// tag::output[]
// print solution for part 1
	println("*******************************")
	println("--- Day 10: Cathode-Ray Tube ---")
	println("*******************************")
	println("Solution for part1")
	println("   $solution1 is the sum of these six signal strengths.")
	println()
// print solution for part 2
	println("*******************************")
	println("Solution for part2")
	println("   $solution2 is the sum of these six signal strengths.")  //16068009780 to high
	println()
// end::output[]

	t1 = System.currentTimeMillis() - t1
	println("puzzle solved in ${t1} ms")
}
