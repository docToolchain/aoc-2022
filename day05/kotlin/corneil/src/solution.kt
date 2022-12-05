data class Instruction(val count: Int, val from: Int, val to: Int)
class Stack<T>(private val elements: MutableList<T> = mutableListOf<T>()) {
  fun empty(): Boolean = elements.isEmpty()
  fun peek(): T = elements[elements.lastIndex]
  fun pop(): T = elements.removeAt(elements.lastIndex)
  fun push(value: T) {
    elements.add(value)
  }

  fun items() = elements.toList()
  fun size() = elements.size
}

fun main() {

  fun readStack(lines: List<String>, stack: Int): Stack<Char> {
    val result = mutableListOf<Char>()
    val offset = stack * 4 + 1
    for (line in lines) {
      if (line.length > offset) {
        val crate = line[offset]
        if (crate in 'A'..'Z') {
          result.add(crate)
        }
      }
    }
    result.reverse()
    println("Stack[${stack + 1}]=${result.joinToString("")}")
    return Stack(result)
  }

  fun loadStacks(input: List<String>): List<Stack<Char>> {
    val lines = mutableListOf<String>()
    for (line in input) {
      if (line.isEmpty()) {
        break;
      }
      lines.add(line)
    }
    val numbers = mutableListOf<Int>()
    for (c in lines.last()) {
      if (c in '1'..'9') {
        numbers.add(c - '1' + 1)
      }
    }
    lines.removeAt(lines.lastIndex)
    val stacks = mutableListOf<Stack<Char>>()
    for (stack in numbers.indices) {
      stacks.add(readStack(lines, stack))
    }
    return stacks.toList()
  }

  fun loadInstructions(input: List<String>): List<Instruction> {
    var foundBlank = false
    val result = mutableListOf<Instruction>()
    for (line in input) {
      if (line.isEmpty()) {
        foundBlank = true
      }
      if (foundBlank) {
        val items = line.split(" ")
        if (items.size > 4) {
          result.add(Instruction(items[1].toInt(), items[3].toInt(), items[5].toInt()))
        }
      }
    }
    return result
  }

  fun perform(stacks: List<Stack<Char>>, instruction: Instruction) {
    println(instruction)
    for (i in 1..instruction.count) {
      val crate = stacks[instruction.from - 1].pop()
      stacks[instruction.to - 1].push(crate)
    }
  }

  fun perform9001(stacks: List<Stack<Char>>, instruction: Instruction) {
    println(instruction)
    val tmp = Stack<Char>()
    for (i in 1..instruction.count) {
      val crate = stacks[instruction.from - 1].pop()
      tmp.push(crate)
    }
    while (!tmp.empty()) {
      stacks[instruction.to - 1].push(tmp.pop())
    }
  }

  fun performAndFindTops(stacks: List<Stack<Char>>, instructions: List<Instruction>): String {
    instructions.forEach {
      perform(stacks, it)
    }
    stacks.forEach { stack -> println("[${stack.items().joinToString("")}]") }
    return stacks.joinToString("") { it.peek().toString() }
  }

  fun performAndFindTops9001(stacks: List<Stack<Char>>, instructions: List<Instruction>): String {
    instructions.forEach {
      perform9001(stacks, it)
    }
    stacks.forEach { stack -> println("[${stack.items().joinToString("")}]") }
    return stacks.joinToString("") { it.peek().toString() }
  }

  fun part1() {
    val testInput = readInput("day05_test")
    val testTops = performAndFindTops(loadStacks(testInput), loadInstructions(testInput))
    println("Part 1 Test Tops = $testTops")
    check(testTops == "CMZ")
    val input = readInput("day05")
    val tops = performAndFindTops(loadStacks(input), loadInstructions(input))
    println("Part 1 Tops = $tops")
    check(tops == "PTWLTDSJV")
  }

  fun part2() {
    val testInput = readInput("day05_test")
    val testTops = performAndFindTops9001(loadStacks(testInput), loadInstructions(testInput))
    println("Part 2 Test Tops = $testTops")
    check(testTops == "MCD")
    val input = readInput("day05")
    val tops = performAndFindTops9001(loadStacks(input), loadInstructions(input))
    println("Part 2 Tops = $tops")
    check(tops == "WZMFVGGZP")
  }
  part1()
  part2()
}
