class GoToJailSquare(identifier: String, pieces: MutableList<Piece>): Square(identifier, pieces) {
    override fun getEffect(): String {
        return "go to jail"
    }
}