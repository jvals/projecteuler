fun main(args: Array<String>) {
    val maxCollectionSize = 6

    val allPossibleDiceSet = CombinationCollection(Digit.values().asList(), maxCollectionSize).combinations.map { it.toSet() }
    for (die in allPossibleDiceSet) {
        println(die)
    }
    println(allPossibleDiceSet.size)

    var distinctArrangements = 0
    for (i in 0 until allPossibleDiceSet.size) {
        for (j in i+1 until allPossibleDiceSet.size) {
            val d1: Set<Digit> = allPossibleDiceSet[i]
            val d2: Set<Digit> = allPossibleDiceSet[j]
            if (SquareNumber.containsAllSquares(d1, d2, 100)) {
                distinctArrangements += 1
            }
        }
    }
    println(distinctArrangements)
}
