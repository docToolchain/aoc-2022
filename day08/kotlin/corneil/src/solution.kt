import kotlin.math.max

fun main() {
  val test = readLines(
    """30373
25512
65332
33549
35390"""
  )
  val input = readFile("day08")

  class Grid(val lines: Array<IntArray>) {
    fun width() = lines[0].size
    fun height() = lines.size
    fun get(x: Int, y: Int) = lines[x][y]
    fun visible(x: Int, y: Int): Boolean {
      val treeHeight = get(x, y)
      var bottom = true
      var top = true
      var left = true
      var right = true
      for (i in 0 until x) {
        val tree = get(i, y)
        if (tree >= treeHeight) {
          top = false
          break
        }
      }
      for (i in x + 1 until height()) {
        if (get(i, y) >= treeHeight) {
          bottom = false
          break
        }
      }
      for (i in y + 1 until width()) {
        if (get(x, i) >= treeHeight) {
          right = false
          break
        }
      }
      for (i in 0 until y) {
        if (get(x, i) >= treeHeight) {
          left = false
          break
        }
      }
      return top || bottom || left || right
    }

    fun scenic(x: Int, y: Int): Int {
      val treeHeight = get(x, y)
      var bottom = 0
      var top = 0
      var left = 0
      var right = 0
      for (i in x + 1 until height()) {
        bottom += 1
        if (get(i, y) >= treeHeight) {
          break
        }
      }
      for (i in x - 1 downTo  0 ) {
        top += 1
        if (get(i, y) >= treeHeight) {
          break
        }
      }
      for (i in y + 1 until width()) {
        right += 1
        if (get(x, i) >= treeHeight) {
          break
        }
      }
      for (i in y - 1 downTo  0 ) {
        left += 1
        if (get(x, i) >= treeHeight) {
          break
        }
      }
      return bottom * left * right * top
    }
  }

  fun parseGrid(input: List<String>): Grid {
    return Grid(input.map { line -> line.toList().map { it - '0' }.toIntArray() }.toTypedArray())
  }

  fun calcVisible(grid: Grid): Int {
    val height = grid.height()
    val width = grid.width()
    var result = (height * 2) + (width * 2) - 4
    for (x in 1 until height - 1) {
      for (y in 1 until width - 1) {
        if (grid.visible(x, y)) {
          result += 1
        }
      }
    }
    return result
  }

  fun calcMaxScenic(grid: Grid): Int {
    val height = grid.height()
    val width = grid.width()
    var result = 0
    for (x in 1 until height - 1) {
      for (y in 1 until width - 1) {
        result = max(result, grid.scenic(x, y))
      }
    }
    return result
  }

  fun part1() {
    val testResult = calcVisible(parseGrid(test))
    println("Part 1 Answer = $testResult")
    check(testResult == 21)
    val result = calcVisible(parseGrid(input))
    println("Part 1 Answer = $result")
    check(result == 1851)
  }

  fun part2() {
    val testResult = calcMaxScenic(parseGrid(test))
    println("Part 2 Answer = $testResult")
    check(testResult == 8)
    val result = calcMaxScenic(parseGrid(input))
    println("Part 1 Answer = $result")
    check(result == 574080)
  }
  println("Day - 08")
  separator()
  part1()
  separator()
  part2()
}
