import utils.readFile
import utils.separator

fun main() {

  val test = readFile("day11_test")
  val input = readFile("day11")

  data class Monkey(
    val number: Int,
    val worriedLevel: Long,
    val items: MutableList<Long>,
    val boredTarget: Int,
    val worriedTarget: Int,
    val expression: (Long) -> Long,
  ) {
    var inspected: Int = 0
      private set
    fun inspect(value: Int) {
      inspected += value
    }
    fun findTarget(boredLevel: Long) = if (boredLevel % worriedLevel != 0L) worriedTarget else boredTarget
    override fun toString(): String {
      return "Monkey(number=$number, inspected=$inspected, worriedLevel=$worriedLevel, boredTarget=$boredTarget, worriedTarget=$worriedTarget, items=$items)"
    }
  }

  val regexMonkey = listOf(
    "Monkey (\\d+):$",
    "  Starting items:\\s*((\\S+(,\\s)*)*)\$",
    "  Operation: new =\\s(\\S+)\\s(\\+|\\*)\\s(\\S+)\$",
    "  Test: divisible by (\\d+)$",
    "    If true: throw to monkey (\\d+)$",
    "    If false: throw to monkey (\\d+)$"
  ).map { it.toRegex() }
  fun parseMonkey(lines: List<String>): Monkey {
    val result = regexMonkey.mapIndexed { index, regex -> regex.find(lines[index]) ?: error("Regex error for ${lines[index]}") }.toTypedArray()

    val items = result[1].groupValues[1].split(",")
      .map { it.trim().toLong() }
      .toMutableList()

    val words = result[2].groupValues.drop(1)
    check(words[0] == "old")
    val isAdd = words[1] == "+"
    val constant = words[2].toLongOrNull()
    val lambda: (Long) -> Long = if (isAdd) { old -> old + (constant ?: old) } else { old -> old * (constant ?: old) }

    return Monkey(
      result[0].groupValues[1].toInt(),
      result[3].groupValues[1].toLong(),
      items,
      result[4].groupValues[1].toInt(),
      result[5].groupValues[1].toInt(),
      lambda
    )
  }

  fun processItems(monkeys: Map<Int, Monkey>, rounds: Int, divisor: Long = 3L): Map<Int, Monkey> {
    // The mod of the total of worriedLevels overcomes the Long overflow
    // using all divisors ensure that it is the smallest value that will
    // still satisfy all the requirements when using a large number of rounds
    val divisors = monkeys.values.map { monkey -> monkey.worriedLevel }.reduce { acc, l -> acc * l * divisor }
    val sorted = monkeys.values.sortedBy { it.number }
    repeat(rounds) {
      sorted.forEach { monkey ->
        monkey.items.forEach { item ->
          val level = monkey.expression(item)
          val bored = level / divisor
          val targetNumber = monkey.findTarget(bored)
          val targetMonkey = monkeys[targetNumber] ?: error("Cannot find target Monkey:$targetNumber")
          targetMonkey.items.add(bored % divisors) // mod to ensure smallest valid value
        }
        monkey.inspect(monkey.items.size)
        monkey.items.clear()
      }
    }
    return monkeys
  }

  fun calcShenanigans1(input: List<String>): Int {
    val monkeys = input.chunked(7).map { parseMonkey(it) }.associateBy { it.number }
    println("Before: ====")
    monkeys.values.forEach { println(it.toString()) }
    val result = processItems(monkeys, 20)
    println("After: ====")
    result.values.forEach { println(it.toString()) }
    return result.values.map { it.inspected }.sortedDescending().take(2).reduce { acc, i -> acc * i }
  }

  fun calcShenanigans2(input: List<String>): Long {
    val monkeys = input.chunked(7).map { parseMonkey(it) }.associateBy { it.number }
    println("Before: ====")
    monkeys.values.forEach { println(it.toString()) }
    val result = processItems(monkeys, 10000, 1)
    println("After: ====")
    result.values.forEach { println(it.toString()) }
    return result.values.map { it.inspected.toLong() }.sortedDescending().take(2).reduce { acc, i -> acc * i }
  }

  fun part1() {
    val testResult = calcShenanigans1(test)
    println("Part 1 Answer = $testResult")
    check(testResult == 10605)
    val result = calcShenanigans1(input)
    println("Part 1 Answer = $result")
    check(result == 151312)
  }

  fun part2() {
    val testResult = calcShenanigans2(test)
    println("Part 2 Answer = $testResult")
    check(testResult == 2713310158L)
    val result = calcShenanigans2(input)
    println("Part 2 Answer = $result")
    check(result == 51382025916L)
  }
  println("Day - 11")
  separator()
  part1()
  separator()
  part2()
}
