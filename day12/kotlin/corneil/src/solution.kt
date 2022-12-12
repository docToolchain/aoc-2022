package day12

import main.utils.Edge
import main.utils.Graph
import main.utils.measureAndPrint
import utils.Coord
import utils.readFile
import utils.readLines

fun main() {

  val test = readLines(
    """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""
  )
  val input = readFile("day12")

  data class Cell(
    val c: Char,
    val pos: Coord
  ) : Comparable<Cell> {

    override fun compareTo(other: Cell): Int {
      var result = c.compareTo(other.c)
      if (result == 0) {
        result = pos.compareTo(other.pos)
      }
      return result
    }

    override fun equals(other: Any?): Boolean {
      if (this === other) return true
      other as Cell
      if (c != other.c) return false
      if (pos != other.pos) return false


      return true
    }

    override fun hashCode(): Int {
      return pos.hashCode() * c.hashCode()
    }

    fun actual(): Char = when (c) {
      'S' -> 'a'
      'E' -> 'z'
      else -> c
    }

    override fun toString(): String {
      return "Cell($c, $pos)"
    }

  }

  fun createGrid(input: List<String>): List<Edge<Cell>> {
    val edges = mutableListOf<Edge<Cell>>()
    val cells = input.mapIndexed { y, line ->
      line.mapIndexed { x, c ->
        Cell(c, Coord(x, input.lastIndex - y))
      }
    }.flatMap { row -> row.map { it } }.associateBy { it.pos }
    cells.values.forEach { cell ->
      cell.pos.surrounds().forEach { coord ->
        val neighbour = cells[coord]
        if (neighbour != null) {
          val height = neighbour.actual() - cell.actual()
          if (height <= 1) {
            edges.add(Edge(cell, neighbour))
          }
        }
      }
    }
    return edges
  }

  fun calculateSteps(
    edges: List<Edge<Cell>>,
    start: Cell,
    end: Cell
  ): Int? {
    val graph = Graph(edges, true)
    val path = graph.findPath(start, end)
    return if (path.isEmpty()) null else path.size - 1
  }

  fun calcSolution1(input: List<String>): Int {
    val edges = createGrid(input)
    val end = edges.map { edge -> edge.c2 }
      .find { it.c == 'E' } ?: error("Cannot find E")
    val start = edges.map { edge -> edge.c1 }
      .find { it.c == 'S' } ?: error("Cannot find S")
    return calculateSteps(edges, start, end)
      ?: error("Cannot find solution from $start to $end")
  }

  fun calcSolution2(input: List<String>): Int {
    val edges = createGrid(input)
    val end = edges.map { edge -> edge.c2 }
      .find { it.c == 'E' } ?: error("Cannot find E")
    return edges.map { edge -> edge.c1 }
      .filter { it.actual() == 'a' }
      .mapNotNull { start -> calculateSteps(edges, start, end) }
      .min()
  }

  fun part1() {
    val testResult = measureAndPrint("Part 1 Test Time: ") {
      calcSolution1(test)
    }
    println("Part 1 Answer = $testResult")
    check(testResult == 31)
    val result = measureAndPrint("Part 1 Time: ") {
      calcSolution1(input)
    }
    println("Part 1 Answer = $result")
    check(result == 339)
  }

  fun part2() {
    val testResult = measureAndPrint("Part 2 Test Time: ") {
      calcSolution2(test)
    }
    println("Part 2 Answer = $testResult")
    check(testResult == 29)
    val result = measureAndPrint("Part 2 Time: ") {
      calcSolution2(input)
    }
    println("Part 2 Answer = $result")
    check(result == 332)
  }
  println("Day - 12")
  part1()
  part2()
}
