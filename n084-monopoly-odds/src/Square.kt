abstract class Square(val identifier: String, val pieces: MutableList<Piece> = mutableListOf(), var hits: Int = 0) {
    fun incrementHits() {
        hits += 1
    }

    abstract fun getEffect(): String
    override fun toString(): String {
        return "Square(identifier='$identifier', pieces=$pieces, hits=$hits)"
    }

}