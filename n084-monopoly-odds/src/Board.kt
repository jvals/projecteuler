class Board(private vararg val pieces: Piece) {
    private val communityDeck: MutableList<String> = mutableListOf(
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "advance to go",
        "go to jail"
    )

    private val chanceDeck: MutableList<String> = mutableListOf(
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "no effect",
        "go back 3 squares",
        "go to next U",
        "go to next R",
        "go to next R",
        "go to R1",
        "go to H2",
        "go to E3",
        "go to C1",
        "advance to go",
        "go to jail"
    )

    init {
        communityDeck.shuffle()
        chanceDeck.shuffle()
    }

    private val squares: List<Square> = initializeSquares()

    fun advancePiece(piece: Piece, nSteps: Int) {
        // Get current position of piece
        val currentPosition: Int = squares.indexOfFirst { it.pieces.contains(piece) }
        // Remove piece from current position
        squares[currentPosition].pieces.remove(piece)
        // Move piece nSteps steps
        val newPosition = Math.floorMod(currentPosition+nSteps, squares.size)
        squares[newPosition].pieces.add(piece)
    }

    fun pieceGoTo(id: String, piece: Piece) {
        squares.first { it.pieces.contains(piece) }.pieces.remove(piece)
        squares.first { it.identifier == id }.pieces.add(piece)
    }

    fun pieceGoToNext(partialId: String, piece: Piece) {
        val currentPosition: Int = squares.indexOfFirst { it.pieces.contains(piece) }
        squares[currentPosition].pieces.remove(piece)

        for (i in currentPosition+1 until squares.size) {
            if (squares[i].identifier.startsWith(partialId)) {
                squares[i].pieces.add(piece)
                return
            }
        }
        for (i in 0 until currentPosition-1) {
            if (squares[i].identifier.startsWith(partialId)) {
                squares[i].pieces.add(piece)
                return
            }
        }
    }

    fun getSquareWithPiece(piece: Piece): Square {
        return squares.first { it.pieces.contains(piece) }
    }

    fun getSquaresSortedByHits(): List<Pair<Square, Int>> {
        return squares.sortedByDescending(Square::hits).map {Pair(it, squares.indexOf(it))}
    }

    private fun initializeSquares(): List<Square> {
        return listOf(
            PlainSquare("GO", pieces.toMutableList(), 0),
            PlainSquare("A1", mutableListOf()),
            CommunitySquare("CC1", mutableListOf(), communityDeck),
            PlainSquare("A2", mutableListOf()),
            PlainSquare("T1", mutableListOf()),
            PlainSquare("R1", mutableListOf()),
            PlainSquare("B1", mutableListOf()),
            ChanceSquare("CH1", mutableListOf(), chanceDeck),
            PlainSquare("B2", mutableListOf()),
            PlainSquare("B3", mutableListOf()),
            PlainSquare("JAIL", mutableListOf()),
            PlainSquare("C1", mutableListOf()),
            PlainSquare("U1", mutableListOf()),
            PlainSquare("C2", mutableListOf()),
            PlainSquare("C3", mutableListOf()),
            PlainSquare("R2", mutableListOf()),
            PlainSquare("D1", mutableListOf()),
            CommunitySquare("CC2", mutableListOf(), communityDeck),
            PlainSquare("D2", mutableListOf()),
            PlainSquare("D3", mutableListOf()),
            PlainSquare("FP", mutableListOf()),
            PlainSquare("E1", mutableListOf()),
            ChanceSquare("CH2", mutableListOf(), chanceDeck),
            PlainSquare("E2", mutableListOf()),
            PlainSquare("E3", mutableListOf()),
            PlainSquare("R3", mutableListOf()),
            PlainSquare("F1", mutableListOf()),
            PlainSquare("F2", mutableListOf()),
            PlainSquare("U2", mutableListOf()),
            PlainSquare("F3", mutableListOf()),
            GoToJailSquare("G2J", mutableListOf()),
            PlainSquare("G1", mutableListOf()),
            PlainSquare("G2", mutableListOf()),
            CommunitySquare("CC3", mutableListOf(), communityDeck),
            PlainSquare("G3", mutableListOf()),
            PlainSquare("R4", mutableListOf()),
            ChanceSquare("CH3", mutableListOf(), chanceDeck),
            PlainSquare("H1", mutableListOf()),
            PlainSquare("T2", mutableListOf()),
            PlainSquare("H2", mutableListOf())
        )
    }
}