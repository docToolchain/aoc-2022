fun readInput(name: String) = File("src", "$name.txt")
    .readLines()

fun main() {
    // tag::part1[]
    fun findCalories(input: List<String>): List<Int> {
        val elves = mutableListOf<Int>()
        var calories = 0
        for (line in input) {
            if (line.isBlank()) {
                elves.add(calories)
                calories = 0
            } else {
                calories += line.toInt()
            }
        }
        elves.add(calories)
        return elves
    }


    fun findMaxCalories(input: List<String>): Int {
        val calories = findCalories(input)
        return calories.max()
    }
    // end::part1[]
    // tag::part2[]
    fun topThree(input: List<String>): Int {
        val calories = findCalories(input)
        return calories.sortedDescending().take(3).sum()
    }
    // end::part2[]
    val testInput = readInput("day01_input1")
    val testMax = findMaxCalories(testInput)
    println("Test Max Calories = $testMax")
    check(testMax == 24000)
    val input = readInput("day01")
    val maxCalories = findMaxCalories(input)
    println("Max Calories = $maxCalories")

    val testTop3 = topThree(testInput)
    println("Test Top3 = $testTop3")
    val top3 = topThree(input)
    println("Top3 = $top3")
}
