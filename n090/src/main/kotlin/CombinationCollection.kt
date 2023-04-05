data class CombinationCollection(private val digits: List<Digit>, private val maxCollectionSize: Int){
    val combinations: MutableList<List<Digit>> = mutableListOf()

    init {
        buildCombo()
    }

    private fun buildCombo(combo: List<Digit> = emptyList(), digits: List<Digit> = this.digits) {
        if (combo.size == maxCollectionSize) {
            combinations.add(combo)
        }
        else {
            digits.forEach {
                buildCombo(combo + it, digits.filter { d -> d > it })
            }
        }
    }
}
