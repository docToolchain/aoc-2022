enum class Rps(val identifiers: String, val shapeScore: Int) {
    ROCK("AX", 1),
    PAPER("BY", 2),
    SCISSORS("CZ", 3)
}

enum class Outcome(val identifier: String, val score: Int) {
    LOSE("X", 0),
    DRAW("Y", 3),
    WIN("Z", 6)
}

data class Hand(val them: Rps, val me: Rps)

object Rules {

    data class Rule(val loser: Rps, val winner: Rps)

    private val beats = mapOf(
        Rps.ROCK to Rule(Rps.SCISSORS, Rps.PAPER),
        Rps.PAPER to Rule(Rps.ROCK, Rps.SCISSORS),
        Rps.SCISSORS to Rule(Rps.PAPER, Rps.ROCK)
    )

    fun winner(shape: Rps): Rps {
        val winner = beats[shape] ?: error("Cannot find $shape")
        return winner.winner
    }

    fun loser(shape: Rps): Rps {
        val loser = beats[shape] ?: error("Cannot find $shape")
        return loser.loser
    }

    fun didIWin(round: Hand): Boolean {
        val shape = loser(round.me)
        return shape == round.them
    }

}

fun main() {

    fun toRps(hint: String): Rps {
        return Rps.values().find { it.identifiers.contains(hint) } ?: error("Invalid RPS $hint")
    }

    fun toOutcome(outcome: String): Outcome {
        return Outcome.values().find { it.identifier == outcome } ?: error("Invalid Outcome:$outcome")
    }

    fun readRounds(input: List<String>): List<Hand> {
        return input.map {
            val hints = it.split(" ")
            Hand(toRps(hints[0]), toRps(hints[1]))
        }
    }

    // choose a shape to match the outcome
    fun chooseShape(shape: Rps, outcome: Outcome): Rps {
        return when (outcome) {
            Outcome.DRAW -> shape
            Outcome.WIN -> Rules.winner(shape)
            Outcome.LOSE -> Rules.loser(shape)
        }
    }

    // convert 2nd to outcome and then find a shape to match outcome given 1st
    fun readRounds2(input: List<String>): List<Hand> {
        return input.map {
            val hints = it.split(" ")
            Pair(toRps(hints[0]), toOutcome(hints[1]))
        }.map {
            Hand(it.first, chooseShape(it.first, it.second))
        }
    }

    fun calcRound(round: Hand): Int {
        return when {
            round.them == round.me -> Outcome.DRAW.score
            Rules.didIWin(round) -> Outcome.WIN.score
            else -> Outcome.LOSE.score
        }
    }

    fun calcScore(round: Hand): Int {
        return round.me.shapeScore + calcRound(round)
    }

    fun calcTotal(rounds: List<Hand>): Int {
        return rounds.sumOf { calcScore(it) }
    }

    fun part1() {

        val testScore = calcTotal(readRounds(readInput("day02_test")))
        println("Test Total = $testScore")
        check(testScore == 15)

        val total = calcTotal(readRounds(readInput("day02")))
        println("Total = $total")

    }

    fun part2() {

        val testScore2 = calcTotal(readRounds2(readInput("day02_test")))
        println("Test Total 2 = $testScore2")
        check(testScore2 == 12)

        val total2 = calcTotal(readRounds2(readInput("day02")))
        println("Total 2 = $total2")
    }

    part1()
    part2()
}
