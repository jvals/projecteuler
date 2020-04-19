import kotlin.random.Random

class Player(val name: String, val piece: Piece) {
    private val lastThreeThrows: MutableList<Pair<Int, Int>> = mutableListOf(
        Pair(1, 2), Pair(1, 2), Pair(1,2)
    )

    private fun lastThreeThrowsIsDoubles(): Boolean {
        return lastThreeThrows[0].first == lastThreeThrows[0].second &&
                lastThreeThrows[1].first == lastThreeThrows[1].second &&
                lastThreeThrows[2].first == lastThreeThrows[2].second
    }

    fun doTurn(board: Board) {
        // n = throw dice
        val firstDice = Random.nextInt(1,5)
        val secondDice = Random.nextInt(1,5)
        val steps = firstDice + secondDice

        lastThreeThrows.add(Pair(firstDice, secondDice))
        lastThreeThrows.removeAt(0)

        // move piece n steps
        if (lastThreeThrowsIsDoubles()) {
            // Go to jail
            board.pieceGoTo("JAIL", piece)
            // Reset last three throws
            lastThreeThrows.clear()
            lastThreeThrows.add(Pair(1, 2))
            lastThreeThrows.add(Pair(2, 1))
            lastThreeThrows.add(Pair(2, 1))
        } else {
            board.advancePiece(piece, steps)
        }

        // check if the square you land on has any actions
        var square: Square = board.getSquareWithPiece(piece)
        var effect: String = square.getEffect()
        while (effect != "no effect") {
            when (effect) {
                "advance to go" -> {
                    board.pieceGoTo("GO", piece)
                }
                "go to jail" -> {
                    board.pieceGoTo("JAIL", piece)
                }
                "go back 3 squares" -> {
                    board.advancePiece(piece, -3)
                }
                "go to next U" -> {
                    board.pieceGoToNext("U", piece)
                }
                "go to next R" -> {
                    board.pieceGoToNext("R", piece)
                }
                "go to R1" -> {
                    board.pieceGoTo("R1", piece)
                }
                "go to H2" -> {
                    board.pieceGoTo("H2", piece)
                }
                "go to E3" -> {
                    board.pieceGoTo("E3", piece)
                }
                "go to C1" -> {
                    board.pieceGoTo("C1", piece)
                }
                else -> {
                    println("Unexpected effect $effect")
                }
            }
            square = board.getSquareWithPiece(piece)
            effect = square.getEffect()
        }

        // execute any required actions
        // increment the number of hits on the final square
        square.incrementHits()
    }
}