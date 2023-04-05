object SquareNumber {
    private val squareNumbers = listOf(1, 4, 9, 16, 25, 36, 49, 64, 81)

    fun containsAllSquares(d1: Collection<Digit>, d2: Collection<Digit>, limit: Int): Boolean {
        val d1Values: List<String> = getValues(d1)


        val d2Values: List<String> = getValues(d2)

        val squaresNotFound = squareNumbers.toMutableList()

        d1Values.forEach { n1 ->
            d2Values.forEach { n2 ->
                val n = (n1 + n2).toInt()
                if (n in squareNumbers) {
                    squaresNotFound.remove(n)
                }
            }
        }

        d2Values.forEach { n1 ->
            d1Values.forEach { n2 ->
                val n = (n1 + n2).toInt()
                if (n in squareNumbers) {
                    squaresNotFound.remove(n)
                }
            }
        }

        return squaresNotFound.isEmpty()
    }

    private fun getValues(digits: Collection<Digit>): List<String> {
        val primaryValues = digits.map { it.value }.toSet()
        val secondaryValues = digits.mapNotNull { it.alias }.toSet()
        val strings = (primaryValues + secondaryValues).map { it.toString() }
        return strings
    }
}