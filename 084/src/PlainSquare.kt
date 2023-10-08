class PlainSquare(identifier: String, pieces: MutableList<Piece>, hits: Int = 0): Square(identifier, pieces, hits) {
    override fun getEffect(): String {
        return "no effect"
    }
}