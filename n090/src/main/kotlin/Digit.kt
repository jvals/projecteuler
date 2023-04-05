enum class Digit(val value: Int, val alias: Int? = null) {
    ZERO(0),
    ONE(1),
    TWO(2),
    THREE(3),
    FOUR(4),
    FIVE(5),
    SIX(6, 9),
    SEVEN(7),
    EIGHT(8),
    NINE(9, 6);
}