package day13

import utils.groupLines
import utils.readFile
import utils.readLines
import utils.separator
import java.util.*

fun main() {
  val test = readLines(
    """[1,1,3,1,1]
       [1,1,5,1,1]
      
       [[1],[2,3,4]]
       [[1],4]
      
       [9]
       [[8,7,6]]
      
       [[4,4],4,4]
       [[4,4],4,4,4]
      
       [7,7,7,7]
       [7,7,7]
      
       []
       [3]
      
       [[[]]]
       [[]]
      
       [1,[2,[3,[4,[5,6,7]]]],8,9]
       [1,[2,[3,[4,[5,6,0]]]],8,9]
       """.trimIndent()
  )
  val input = readFile("day13")

  abstract class Item
  data class Value(val value: Int) : Item() {
    fun compareTo(right: Value): Int {
      return value.compareTo(right.value)
    }

    override fun toString(): String {
      return value.toString()
    }
  }

  data class Packet(val items: List<Item>) : Item(), List<Item> by items {
    constructor(value: Item) : this(listOf(value)) {}
  }

  fun parsePacket(input: String): Packet {
    val value = StringBuilder()
    var list: MutableList<Item> = mutableListOf()

    fun checkAdd() {
      if (value.isNotBlank()) {
        list.add(Value(value.toString().toInt()))
      }
      value.clear()
    }

    val lists = Stack<List<Item>>()
    var index = 1 // skip first [
    while (index < input.lastIndex) {
      when (val c = input[index]) {
        ',', ' ' -> checkAdd()
        '[' -> {
          lists.push(list)
          list = mutableListOf()
        }

        ']' -> {
          checkAdd()
          val v = list
          list = lists.pop().toMutableList()
          list.add(Packet(v))
        }

        '-' -> value.append('-')
        '+' -> value.append('+')
        else -> {
          if (c.isDigit()) {
            value.append(c)
          } else {
            error("Unexpected $c")
          }
        }
      }
      index += 1
    }
    checkAdd()
    return Packet(list.toList())
  }


  fun Packet.compareTo(right: Packet): Int {
    var result = 0
    for (index in indices) {
      val item = this[index]
      val rightItem = right.getOrNull(index)
      result = when {
        rightItem == null -> 1
        item is Value && rightItem is Value -> item.compareTo(rightItem)
        item is Packet && rightItem is Value -> item.compareTo(Packet(rightItem))
        item is Packet && rightItem is Packet -> item.compareTo(rightItem)
        item is Value && rightItem is Packet -> Packet(item).compareTo(rightItem)
        else -> 1
      }
      if (result != 0) {
        break
      }
    }
    if (result == 0 && size < right.size) {
      result = -1
    }
    return result
  }

  fun Item.isOrdered(right: Item?): Int {
    return when {
      right == null -> 1
      this is Value && right is Value -> compareTo(right)
      this is Packet && right is Packet -> this.compareTo(right)
      this is Packet && right is Value -> this.compareTo(Packet(right))
      this is Value && right is Packet -> Packet(this).compareTo(right)
      else -> 1
    }
  }

  fun isIndexOrdered(index: Int, input: List<String>): Boolean {
    check(input.size == 2)
    val (left, right) = input.map { parsePacket(it) }
    var result = 0
    for (index in left.indices) {
      val item = left[index]
      val rightItem = if (index <= right.lastIndex)
        right[index] else null
      result = item.isOrdered(rightItem)
      if (result != 0) {
        break
      }
    }
    val ordered = result <= 0
    if (result == 0) {
      println("Pair $index: was the same")
    } else if (result < 0) {
      println("Pair $index: left was lower")
    }
    return ordered
  }

  fun calcSolution1(input: List<List<String>>): Int {
    return input.mapIndexedNotNull { index, inputs ->
      if (isIndexOrdered(index, inputs)) index + 1 else null
    }.sum()
  }

  fun calcSolution2(input: List<String>): Int {
    val extra1 = "[[2]]"
    val extra2 = "[[6]]"
    val packets = input.filter { it.isNotBlank() }.toMutableList()
    packets.add(extra1)
    packets.add(extra2)
    val sorted = packets.map { it to parsePacket(it) }
      .sortedWith { o1, o2 -> o1.second.compareTo(o2.second) }
      .map { it.first }
    val index1 = sorted.indexOf(extra1) + 1
    val index2 = sorted.indexOf(extra2) + 1
    return index1 * index2
  }

  fun part1() {
    val testResult = calcSolution1(groupLines(test))
    println("Part 1 Test Answer = $testResult")
    check(testResult == 13)
    separator()
    val result = calcSolution1(groupLines(input))
    println("Part 1 Answer = $result")
    check(result == 4643)
  }

  fun part2() {
    val testResult = calcSolution2(test)
    println("Part 2 Test Answer = $testResult")
    check(testResult == 140)
    separator()
    val result = calcSolution2(input)
    println("Part 2 Answer = $result")
    check(result == 21614)
  }
  println("Day - 13")
  part1()
  separator()
  part2()
}
