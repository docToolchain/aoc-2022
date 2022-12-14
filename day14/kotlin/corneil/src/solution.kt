package day14

import main.utils.measureAndPrint
import main.utils.scanInts
import utils.Coord
import utils.readFile
import utils.readLines
import utils.separator
import kotlin.math.max
import kotlin.math.min

enum class Substance(val c: Char) {
  AIR('.'),
  ROCK('#'),
  SAND('o')
}

enum class Direction {
  STOP,
  NONE,
  DOWN,
  LEFT,
  RIGHT,
  ABYSS
}

object Depth {
  private val depth = ThreadLocal.withInitial { 0 }
  var maxDepth = 0
  fun enter() {
    depth.set(depth.get() + 1)
    maxDepth = max(maxDepth, depth.get())
  }

  fun exit() {
    depth.set(depth.get() - 1)
  }
}

fun main() {

  val test = readLines(
    """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"""
  )
  val input = readFile("day14")

  open class Grid(val cells: MutableMap<Coord, Substance> = mutableMapOf()) : MutableMap<Coord, Substance> by cells {
    open val maxX: Int get() = cells.keys.maxOf { it.x }
    open val minX: Int get() = cells.keys.minOf { it.x }
    open val maxY: Int get() = cells.keys.maxOf { it.y }
    open val minY: Int = 0
    open fun stretch() {}
    fun countSand() = cells.values.count { it == Substance.SAND }
    open fun isInGrid(pos: Coord): Boolean = pos.x in minX..maxX
        && pos.y in minY..maxY

    fun getSubStance(pos: Coord) = getOrDefault(pos, Substance.AIR)
    fun isAir(pos: Coord) = getSubStance(pos) == Substance.AIR
    fun isTarget(pos: Coord) = isInGrid(pos) && isAir(pos)
    fun canDrop(pos: Coord): Pair<Coord, Direction> {
      Depth.enter()
      try {
        if (!isInGrid(pos.left()) || !isInGrid(pos.right())) {
          return Pair(pos, Direction.ABYSS)
        }
        if (isInGrid(pos.bottom())) {
          if (isTarget(pos.bottom())) {
            return canDrop(pos.bottom())
          } else {
            if (isTarget(pos.bottom().left())) {
              return canDrop(pos.bottom().left())
            } else if (isTarget(pos.bottom().right())) {
              return canDrop(pos.bottom().right())
            }
            if (isTarget(pos)) {
              return Pair(pos, Direction.DOWN)
            }
          }
        }
        return Pair(pos, Direction.STOP)
      } finally {
        Depth.exit()
      }
    }

    fun print(entry: Coord?) {
      val zX = maxX
      val aX = minX
      val zY = maxY
      val aY = minY
      for (y in aY..zY) {
        for (x in aX..zX) {
          val pos = Coord(x, y)
          if (entry != null && pos == entry) {
            print('+')
          } else {
            print(getOrDefault(pos, Substance.AIR).c)
          }
        }
        println()
      }
    }
  }

  class InfiniteGird(cells: MutableMap<Coord, Substance>, val level: Int) : Grid(cells) {
    override val maxY: Int get() = level
    override fun get(key: Coord): Substance? {
      if (key.y == level) {
        return Substance.ROCK
      }
      return super.get(key)
    }

    override fun getOrDefault(key: Coord, defaultValue: Substance): Substance {
      if (key.y == level) {
        return Substance.ROCK
      }
      return super.getOrDefault(key, defaultValue)
    }

    override fun isInGrid(pos: Coord): Boolean = pos.y <= level
    override fun stretch() {
      val zX = (cells.keys.filter { it.y < level }.maxOfOrNull { it.x } ?: maxX) + 1
      val aX = (cells.keys.filter { it.y < level }.minOfOrNull { it.x } ?: minX) - 1
      for (x in aX..zX) {
        val pos = Coord(x, level)
        if (!containsKey(pos)) {
          cells[pos] = Substance.ROCK
        }
      }
    }
  }

  fun loadStructure(lines: List<String>): Grid {
    val grid = mutableMapOf<Coord, Substance>()
    val scans = lines.map {
      it.scanInts()
        .windowed(2, 2)
        .map { line -> Coord(line[0], line[1]) }
    }
    scans.forEach { scan ->
      scan.windowed(2)
        .forEach { pair ->
          check(pair.size == 2)
          val prev = pair[0]
          val curr = pair[1]
          val diff = curr - prev
          check((diff.x == 0 && diff.y != 0) || (diff.x != 0 && diff.y == 0)) { "Expected one 0 in $diff" }
          if (diff.y == 0) {
            for (x in min(curr.x, prev.x)..max(curr.x, prev.x)) {
              val pos = Coord(x, curr.y)
              if (!grid.containsKey(pos)) {
                grid[pos] = Substance.ROCK
              }
            }
          } else {
            for (y in min(curr.y, prev.y)..max(curr.y, prev.y)) {
              val pos = Coord(curr.x, y)
              if (!grid.containsKey(pos)) {
                grid[pos] = Substance.ROCK
              }
            }
          }
        }
    }
    return Grid(grid)
  }

  fun isStop(direction: Direction) = direction == Direction.STOP
      || direction == Direction.NONE
      || direction == Direction.ABYSS

  fun dropSand(grid: Grid, entry: Coord): Pair<Coord, Boolean> {
    val drop = grid.canDrop(entry)
    if (drop.second == Direction.ABYSS) {
      return Pair(drop.first, false)
    }
    return Pair(drop.first, !isStop(drop.second))
  }

  fun simulateSand(grid: Grid, start: Coord, print: Boolean): Int {
    do {
      val result = dropSand(grid, start)
      if (result.second) {
        grid[result.first] = Substance.SAND
      }
    } while (result.second)
    return grid.countSand()
  }

  fun calcSand(input: List<String>, print: Boolean): Int {
    val grid = loadStructure(input)
    val entry = Coord(500, 0)
    if(print) {
      grid.print(entry)
    }
    simulateSand(grid, entry, print)
    if(print) {
      grid.print(entry)
    }
    return grid.countSand()
  }

  fun calcInfiniteSand(input: List<String>, print: Boolean): Int {
    val grid = loadStructure(input)
    val entry = Coord(500, 0)
    val infiniteGrid = InfiniteGird(grid.toMutableMap(), grid.maxY + 2)
    if (print) {
      grid.print(entry)
    }
    infiniteGrid.stretch()
    val sand = simulateSand(infiniteGrid, entry, print)
    if (print) {
      infiniteGrid.print(null)
    }
    return sand
  }

  fun part1() {
    val testResult = calcSand(test, false)
    println("Part 1 Test Answer = $testResult")
    check(testResult == 24) { "Expected 24 not $testResult" }
    val result = measureAndPrint("Test 1 Time: ") {
      calcSand(input, false)
    }
    println("Part 1 Answer = $result")
  }

  fun part2() {
    val testResult = calcInfiniteSand(test, false)
    println("Part 2 Test Answer = $testResult")
    check(testResult == 93) { "Expected 93 not $testResult" }
    val result = measureAndPrint("Test 2 Time: ") {
      calcInfiniteSand(input, false)
    }
    println("Part 2 Answer = $result")
    check(result == 22499) { "Expected 22499 not $result" }
  }
  println("Day - 14")
  part1()
  part2()
}
