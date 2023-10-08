import Digit.*
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class SquareNumberTest {
    @Test
    fun `Two dice can contain the first 100 square numbers`() {
        val d1 = listOf(ZERO, FIVE, SIX, SEVEN, EIGHT, NINE)
        val d2 = listOf(ONE, TWO, THREE, FOUR, EIGHT, NINE)
        assertTrue(SquareNumber.containsAllSquares(d1, d2, 100))
    }
}