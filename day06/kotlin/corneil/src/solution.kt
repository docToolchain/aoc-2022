fun main() {
  fun findPacketStart(input: String, packetSize: Int): Int {
    val data = input.toList()
    val packet = mutableListOf<Char>()

    for (index in data.indices) {
      packet.add(data[index])
      if (packet.size > packetSize) {
        packet.removeAt(0)
      }
      if (packet.size == packetSize && packet.toSet().size == packetSize) {
        println("Packet=$packet")
        return index + 1
      }
    }
    return input.length
  }

  val input = readFileToString("day06")

  fun part1() {
    val testStart = findPacketStart("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)
    println("Part1 Test Start = $testStart")
    check(testStart == 10)
    val start = findPacketStart(input, 4)
    println("Part1 Start = $start")
    check(start == 1965)
  }

  fun part2() {
    val testStart = findPacketStart("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)
    println("Part 2 Test Message = $testStart")
    check(testStart == 29)
    val start = findPacketStart(input, 14)
    println("Part 2 Start = $start")
    check(start == 2773)
  }
  part1()
  part2()
}
