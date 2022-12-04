fun main() {
  fun convertRange(input: String): IntRange {
    val values = input.split("-")
    return values[0].toInt()..values[1].toInt()
  }

  fun convertRanges(input: List<String>): List<Pair<Set<Int>, Set<Int>>> =
    input.map { it.split(",") }
      .map { Pair(convertRange(it[0]).toSet(), convertRange(it[1]).toSet()) }

  fun calcContains(ranges: List<Pair<Set<Int>, Set<Int>>>): Int {
    return ranges.count {
      it.first.containsAll(it.second) || it.second.containsAll(it.first)
    }
  }

  fun calcOverlap(ranges: List<Pair<Set<Int>, Set<Int>>>): Int {
    return ranges.count { it.first.intersect(it.second).isNotEmpty() }
  }

  fun part1() {
    val testInput = readInput("day04_test")
    val testCount = calcContains(convertRanges(testInput))
    println("Part 1 Test Count = $testCount")
    check(testCount == 2)

    val input = readInput("day04")
    val count = calcContains(convertRanges(input))
    println("Part 1 Count = $count")
    check(count == 524)
  }

  fun part2() {
    val testInput = readInput("day04_test")
    val testCount = calcOverlap(convertRanges(testInput))
    println("Part 2 Test Count = $testCount")
    check(testCount == 4)

    val input = readInput("day04")
    val count = calcOverlap(convertRanges(input))
    println("Part 2 Count = $count")
    check(count == 798)
  }
  part1()
  part2()
}
