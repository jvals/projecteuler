def is_a_less_than_b(a, b):
    if b == None:
        return False

    romanPrimitives = ['I', 'V', 'X', 'L', 'C', 'D', 'M']

    return romanPrimitives.index(a) < romanPrimitives.index(b)

def valueOfRomanNumeral(num):
    romanPrimitives = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C':100, 'D':500, 'M':1000}
    return romanPrimitives[num]

def convertRomanNumeralToDecimal(num):
    highest = 'I'
    total = 0
    for i in range(len(num)-1, -1, -1):
        char = num[i]
        if valueOfRomanNumeral(char) > valueOfRomanNumeral(highest):
            highest = char
        if is_a_less_than_b(char, highest):
            total -= valueOfRomanNumeral(char)
        else:
            total += valueOfRomanNumeral(char)
    return total


def convertDecimalDigitToRomanDigit(digit):
    if digit == 1:
        return 'I'
    if digit == 2:
        return 'II'
    if digit == 3:
        return 'III'
    if digit == 4:
        return 'IV'
    if digit == 5:
        return 'V'
    if digit == 6:
        return 'VI'
    if digit == 7:
        return 'VII'
    if digit == 8:
        return 'VIII'
    if digit == 9:
        return 'IX'
    if digit == 10:
        return 'X'
    if digit == 20:
        return 'XX'
    if digit == 30:
        return 'XXX'
    if digit == 40:
        return 'XL'
    if digit == 50:
        return 'L'
    if digit == 60:
        return 'LX'
    if digit == 70:
        return 'LXX'
    if digit == 80:
        return 'LXXX'
    if digit == 90:
        return 'XC'
    if digit == 100:
        return 'C'
    if digit == 200:
        return 'CC'
    if digit == 300:
        return 'CCC'
    if digit == 400:
        return 'CD'
    if digit == 500:
        return 'D'
    if digit == 600:
        return 'DC'
    if digit == 700:
        return 'DCC'
    if digit == 800:
        return 'DCCC'
    if digit == 900:
        return 'CM'
    if digit == 1000:
        return 'M'

    # multiple of 1000
    if digit % 1000 == 0:
        return 'M' * (digit // 1000)

    raise Exception(f'Unable to convert digit {digit} to roman numeral')


def convertDecimalToRomanNumeral(decimal):
    reversedRomanNumeral = ''
    tens = 1
    while decimal > 0:
        digit = (decimal % 10) * tens
        character = convertDecimalDigitToRomanDigit(digit)
        reversedRomanNumeral = character + reversedRomanNumeral
        decimal //= 10
        tens *= 10
    return reversedRomanNumeral


def minimizeRomanNumeral(num):
    decimal = convertRomanNumeralToDecimal(num)
    minimalNum = convertDecimalToRomanNumeral(decimal)
    return minimalNum


def main():
    print(convertDecimalDigitToRomanDigit(4000))
    print(convertDecimalToRomanNumeral(16))
    print(convertDecimalToRomanNumeral(1072))
    print(convertRomanNumeralToDecimal('XVI'))
    print(convertRomanNumeralToDecimal('XIV'))
    print(convertRomanNumeralToDecimal('MLXXII'))

    total_saved = 0
    with open('p089_roman.txt') as f:
        for line in f:
            line = line.strip()
            minimalForm = minimizeRomanNumeral(line)
            print(line, minimalForm)
            total_saved += (len(line) - len(minimalForm))

    print(total_saved)

main()
