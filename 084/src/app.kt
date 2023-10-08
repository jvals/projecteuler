import java.text.DecimalFormat

fun main() {
    // Set up player
    val player: Player = Player("jvals", Piece("red"))
    // Set up board
    val gameBoard: Board = Board(player.piece)
    // Run player on board for N steps
    val N = 10000000
    for (x in 0 until N) {
        player.doTurn(gameBoard)
    }
    // Extract top three most popular squares
    val squaresSortedByHits = gameBoard.getSquaresSortedByHits()
    println("Total hits: ${squaresSortedByHits.sumBy { it.first.hits }}")
    println(squaresSortedByHits.map { "${it.first.identifier} ${it.first.hits.toDouble() / N.toDouble() * 100.0}%" })
    // Create six-digit modal string from previous result
    val first: String = "%02d".format(squaresSortedByHits[0].second)
    val second: String = "%02d".format(squaresSortedByHits[1].second)
    val third: String = "%02d".format(squaresSortedByHits[2].second)
    val sixDigitModalString: String = first+second+third
    println(sixDigitModalString)
}