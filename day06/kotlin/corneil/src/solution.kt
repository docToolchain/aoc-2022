fun main() {
  fun findUniquePacketEnd(input: String, packetSize: Int): Int {
    return input.toList()
      .windowed(packetSize)
      .indexOfFirst { it.toSet().size == packetSize } + packetSize
  }

  val input = readFileToString("day06")

  fun part1() {
    val testStart = findUniquePacketEnd("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
    println("Part1 Test Start = $testStart")
    check(testStart == 10)
    val start = findUniquePacketEnd(input, 4)
    println("Part1 Start = $start")
    check(start == 1965)
  }

  fun part2() {
    val testStart = findUniquePacketEnd("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
    println("Part 2 Test Message = $testStart")
    check(testStart == 29)
    val start = findUniquePacketEnd(input, 14)
    println("Part 2 Start = $start")
    check(start == 2773)
  }
  part1()
  part2()
}
