class ChanceSquare(identifier: String, pieces: MutableList<Piece>, private val deck: MutableList<String>): Square(identifier, pieces) {
    override fun getEffect(): String {
        val card: String = deck[0]
        deck.removeAt(0)
        deck.add(card)
        return card
    }
}