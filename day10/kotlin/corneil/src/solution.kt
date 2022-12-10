import utils.*

sealed class OpCode(val prefix: String) {
  object NOOP: OpCode("noop")
  data class ADDX(val value: Int): OpCode("addx")
}

fun main() {
  val test = readFile("day10_test")
  val input = readFile("day10")

  fun parseInstructions(lines: List<String>): List<OpCode> {
    return lines.map { ins ->
      val words = ins.split(" ")
      when (words[0]) {
        "noop" -> OpCode.NOOP
        "addx" -> OpCode.ADDX(words[1].toInt())
        else -> error("Unknown instruction: $ins")
      }
    }
  }

  class Processor(val processing: Processor.() -> Unit) {
    var clock = 0
      private set

    fun tick() {
      clock += 1
      processing(this)
    }

    var regX: Int = 1
      private set

    fun execute(instructions: List<OpCode>) {
      instructions.forEach { ins ->
        when (ins) {
          is OpCode.NOOP -> tick()
          is OpCode.ADDX -> addX(ins)
        }
      }
    }

    private fun addX(ins: OpCode.ADDX) {
      tick()
      val value = ins.value
      tick()
      regX += value
    }
  }


  fun calcSignalStrength(instructions: List<OpCode>): Int {
    var signalValue = 0
    val cpu = Processor {
      if (clock % 40 == 20) {
        signalValue += regX * clock
      }
    }
    cpu.execute(instructions)
    return signalValue
  }

  fun renderCrt(instructions: List<OpCode>): List<String> {
    val pixels = mutableListOf<Char>()
    val crt = mutableListOf<String>()
    val cpu = Processor {
      val sprite = regX - 1
      val cursor = (clock - 1) % 40
      if (cursor >= sprite && cursor <= sprite + 2) {
        pixels.add('#')
      } else {
        pixels.add('.')
      }
      if (cursor == 39) {
        crt.add(pixels.joinToString(""))
        pixels.clear()
      }
    }
    cpu.execute(instructions)

    return crt.toList()
  }

  fun part1() {
    val testResult = calcSignalStrength(parseInstructions(test))
    println("Part 1 Answer = $testResult")
    check(testResult == 13140)
    val result = calcSignalStrength(parseInstructions(input))
    println("Part 1 Answer = $result")
  }

  val expectedTestCrt = readLines(
    """##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."""
  )
  val expectedCrt = readLines(
    """####..##....##..##..###....##.###..####.
#....#..#....#.#..#.#..#....#.#..#.#....
###..#.......#.#..#.#..#....#.#..#.###..
#....#.......#.####.###.....#.###..#....
#....#..#.#..#.#..#.#....#..#.#.#..#....
#.....##...##..#..#.#.....##..#..#.####."""
  )

  fun part2() {
    val testCrt = renderCrt(parseInstructions(test))
    println("Part 2 Test")
    testCrt.forEach { println(it) }
    check(testCrt == expectedTestCrt)
    val crt = renderCrt(parseInstructions(input))
    println("Part 2")
    crt.forEach { println(it.replace('.',' ')) }
    check(crt == expectedCrt)
  }
  println("Day - 10")
  separator()
  part1()
  separator()
  part2()
}
