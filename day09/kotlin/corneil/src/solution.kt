import kotlin.math.abs
import kotlin.math.max
import kotlin.math.min
import kotlin.math.sign

fun main() {

  val test = """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""
  val test2 = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""
  val input = readFile("day09")

  data class Step(val direction: Char, val distance: Int)
  data class Position(val row: Int, val col: Int) {
    fun left() = Position(row, col - 1)
    fun right() = Position(row, col + 1)
    fun above() = Position(row - 1, col)
    fun below() = Position(row + 1, col)
    fun distance(pos: Position): Int = max(abs(pos.row - row), abs(pos.col - col))
    operator fun minus(pos: Position) = Position(row - pos.row, col - pos.col)
    operator fun plus(pos: Position) = Position(row + pos.row, col + pos.col)
    fun sign(): Position = Position(row.sign,col.sign)
    fun move(direction: Char): Position {
      return when (direction) {
        'U' -> above()
        'D' -> below()
        'L' -> left()
        'R' -> right()
        else -> error("Expected one of U,D,L,R not ${direction}")
      }
    }

    override fun toString(): String {
      return "(col=$col, row=$row)"
    }
  }

  fun parseSteps(input: List<String>): List<Step> {
    return input.map {
      val words = it.split(" ")
      Step(words[0][0], words[1].toInt())
    }
  }

  class Rope(val knots: Array<Position>) {
    val head: Position
      get() = knots[0]
    val tail: Position
      get() = knots[knots.lastIndex]
    operator fun set(index: Int, pos: Position) { knots[index] = pos }
    operator fun get(index: Int) = knots[index]
    fun indexOf(pos: Position) = knots.indexOf(pos)
    override fun toString() = knots.contentToString()
  }

  fun print(
    steps: List<Step>,
    visited: Set<Position>,
    start: Position,
    rope: Rope,
    knots: Int,
    visitsOnly: Boolean = false
  ) {
    var maxRow = 0
    var minRow = 0
    var maxCol = 0
    var minCol = 0
    var dummy = Position(0, 0)
    steps.forEach { step ->
      repeat(step.distance) {
        dummy = dummy.move(step.direction)
        maxRow = max(maxRow, dummy.row)
        maxCol = max(maxCol, dummy.col)
        minRow = min(minRow, dummy.row)
        minCol = min(minCol, dummy.col)
      }
    }

    for (row in minRow..maxRow) {
      val covered = mutableSetOf<Int>()
      for (col in minCol..maxCol) {
        val pos = Position(row, col)
        if (visitsOnly) {
          if (pos == start) {
            print('s')
          } else {
            print(if (visited.contains(pos)) '#' else '.')
          }
        } else {
          val index = rope.indexOf(pos)
          when {
            index == 0 -> print('H')
            index > 0 -> print(if (knots == 2) 'T' else index.toString())
            pos == start -> print('s')
            else -> print(if (visited.contains(pos)) '#' else '.')
          }
          if(index >= 0 && covered.isEmpty()) {
            if(pos == start) {
              covered.add(rope.knots.size) // indicate 's'
            }
            covered.add(index)
            for (i in index..rope.knots.lastIndex) {
              if (pos == rope[i]) {
                covered.add(i)
              }
            }
            if(covered.size == 1) {
              covered.clear()
            }
          }
        }
      }
      if(covered.size > 1) {
        print("  (")
        val coveredKnots = covered.sorted()
        coveredKnots.forEachIndexed { index, knot ->
          if(index == 1) {
            print(" covers ")
          } else if(index > 1) {
            print(" ,")
          }
          when(knot) {
            0 -> print('H')
            rope.knots.lastIndex -> print('T')
            rope.knots.size -> print('s')
            else -> print("$knot")
          }
        }
        println(')')
      }
      println()
    }
  }

  fun calcVisits(steps: List<Step>, knots: Int, printStep: Boolean): Int {
    val rope = Rope(Array(knots) { Position(0, 0) })
    val start = rope.head
    val visited = mutableSetOf<Position>()
    visited.add(rope.tail)
    steps.forEach { step ->
      for (i in 0 until step.distance) {
        rope[0] = rope.head.move(step.direction)
        for (index in 1 until knots) {
          val prev = rope[index - 1]
          val next = rope[index]

          if (prev != next && prev.distance(next) > 1) {
            val diff = prev - next
            rope[index] = next + diff.sign()
          }
        }
        visited.add(rope.tail)
        if (printStep) {
          println()
          print(steps, visited, start, rope, knots, false)
        }
      }
    }
    println()
    print(steps, visited, start, rope, knots, true)
    return visited.size
  }

  fun part1() {
    val testResult = calcVisits(parseSteps(readLines(test)), 2, true)
    println("Part 1 Answer = $testResult")
    check(testResult == 13)
    val result = calcVisits(parseSteps(input), 2, false)
    println("Part 1 Answer = $result")
    check(result == 6503)
  }

  fun part2() {
    check(calcVisits(parseSteps(readLines(test)), 10, true) == 1)
    val testResult = calcVisits(parseSteps(readLines(test2)), 10, true)
    println("Part 2 Answer = $testResult")
    check(testResult == 36)
    val result = calcVisits(parseSteps(input), 10, false)
    println("Part 2 Answer = $result")
    check(result == 2724)
  }

  println("Day - 09")
  separator()
  part1()
  separator()
  part2()
}
