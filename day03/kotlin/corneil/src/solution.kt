fun main() {

  fun calcPriority(value: Char): Int = when (value) {
    in 'a'..'z' -> value - 'a' + 1
    in 'A'..'Z' -> value - 'A' + 27
    else -> error("Invalid input $value")
  }

  fun calcRucksacks(input: List<String>): Int =
    input.map { 
        Pair(it.substring(0 until it.length / 2), it.substring(it.length / 2))
      }
      .map { it.first.toSet().intersect(it.second.toSet()).first() }
      .sumOf { calcPriority(it) }

  fun calcBadges(input: List<String>): Int =
    input.mapIndexed { index, s -> index / 3 to s }
      .groupBy({ it.first }, { it.second })
      .map { e -> e.value.map { it.toSet() }
          .reduce { a, b -> a.intersect(b) }
          .first() 
      }
      .sumOf { calcPriority(it) }

  fun part1() {
    val testPriorities = calcRucksacks(readInput("day03_test"))
    println("Test Priorities = $testPriorities")
    check(testPriorities == 157)

    val priorities = calcRucksacks(readInput("day03"))
    println("Priorities = $priorities")
    check(priorities == 8123)
  }

  fun part2() {
    val testPriorities = calcBadges(readInput("day03_test"))
    println("Test Priorities = $testPriorities")
    check(testPriorities == 70)

    val priorities = calcBadges(readInput("day03"))
    println("Priorities = $priorities")
    check(priorities == 2620)
  }
  part1()
  part2()
}
