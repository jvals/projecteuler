# https://projecteuler.net/problem=54
# Poker hands
# Problem 54

def readf(path):
	p1 = []
	p2 = []
	with open(path) as doc:
		for line in doc:
			# Strip away the newline and the string into a list
			both_hands = line.strip().split(" ")
			p1.append(both_hands[:5]) # Put the first 5 cards into player1 list
			p2.append(both_hands[5:]) # Put the last 5 cards into player2 list 
	return p1,p2

def p1_best_hand(p1,p2):
	p1Score = 10
	p2Score = 10

	p1 = sort_cards(p1)
	p2 = sort_cards(p2)

	if royal_flush(p1) and not royal_flush(p2):
		return True
	if royal_flush(p2) and not royal_flush(p1):
		return False



	if straight_flush(p1) and straight_flush(p2):
		if straight(p1)[1] > straight(p2)[1]:
			return True
		elif straight_flush(p1)[1] < straight_flush(p2)[1]:
			return False
	if straight_flush(p1) and not straight_flush(p2):
		return True
	if straight_flush(p2) and not straight_flush(p1):
		return False



	if four_of_a_kind(p1) and four_of_a_kind(p2):
		if four_of_a_kind(p1)[1] > four_of_a_kind(p2)[1]:
			return True
		elif four_of_a_kind(p1)[1] < four_of_a_kind(p2)[1]:
			return False
	if four_of_a_kind(p1) and not four_of_a_kind(p2):
		return True
	if four_of_a_kind(p2) and not four_of_a_kind(p1):
		return False    



	if full_house(p1) and full_house(p2):
		if full_house(p1)[1] > full_house(p2)[1]:
			return True
		elif full_house(p1)[1] < full_house(p2)[1]:
			return False
	if full_house(p1) and not full_house(p2):
		return True
	if full_house(p2) and not full_house(p1):
		return False
	


	if flush(p1) and flush(p2):
		if flush(p1)[1] > flush(p2)[1]:
			return True
		elif flush(p1)[1] < flush(p2)[1]:
			return False
	if flush(p1) and not flush(p2):
		return True
	if flush(p2) and not flush(p1):
		return False

	

	if straight(p1) and straight(p2):
		if straight(p1)[1] > straight(p2)[1]:
			return True
		elif straight(p1)[1] < straight(p2)[1]:
			return False

	if straight(p1) and not straight(p2):
		return True
	if straight(p2) and not straight(p1):
		return False

	


	if three_of_a_kind(p1) and three_of_a_kind(p2):
		if three_of_a_kind(p1)[1] > three_of_a_kind(p2)[1]:
			return True
		elif three_of_a_kind(p1)[1] < three_of_a_kind(p2)[1]:
			return False

	if three_of_a_kind(p1) and not three_of_a_kind(p2):
		return True
	if three_of_a_kind(p2) and not three_of_a_kind(p1):
		return False

	

	if two_pairs(p1) and two_pairs(p2):
		if two_pairs(p1)[1] > two_pairs(p2)[1]:
			return True
		elif two_pairs(p1)[1] < two_pairs(p2)[1]:
			return False

	if two_pairs(p1) and not two_pairs(p2):
		return True
	if two_pairs(p2) and not two_pairs(p1):
		return False


	if high_card(p1) > high_card(p2):
		return True
	else:
		return False
	

def royal_flush(p):
	""" 
	Ten, Jack, Queen, King, Ace, in same suit.
	"""
	suit = p[0][1]
	for i,card in enumerate(['T','J','Q','K','A']):
		if p[i] != card+suit:
			return False
	return True

def straight_flush(p):
	"""
	All cards are consecutive values of same suit.
	"""
	deck = ['2','3','4','5','6','7','8','9','T','J','Q','K','A']
	suit = p[0][1]
	first = p[0][0]
	deck_position = deck.index(first)
	if deck_position > len(deck)-5:
		return 0
	for i,card in enumerate(deck[deck_position:deck_position+5]):
		if p[i] != card+suit:
			return False
	return True,deck_position


def four_of_a_kind(p):
	deck_weights = get_deck_weights()
	first = p[0][0]
	second = p[1][0]
	values = [c[0] for c in p]
	if values.count(first) == 4:
		return True,deck_weights[first]
	elif values.count(second) == 4:
		return True,deck_weights[second]
	else:
		return False

def full_house(p):
	# 'Three of a kind' can't tie with another 'three of a kind'
	if three_of_a_kind(p) and one_pair(p):
		return True,(three_of_a_kind(p)[1])
	else:
		return False

def flush(p):
	deck_weights = get_deck_weights()
	values = [deck_weights[c[0]] for c in p]
	suits = (card[1] for card in p)
	if len(set(suits)) <= 1: # If all elements are equal
		return True,max(values) # Only check high-card, hopefully this is enough
	else:
		return False


def straight(p):
	deck = ['2','3','4','5','6','7','8','9','T','J','Q','K','A']
	first = p[0][0]
	deck_position = deck.index(first)
	if deck_position > len(deck)-5:
		return 0

	# iterate from through the deck from the position of your first card
	for i,card_value in enumerate(deck[deck_position:deck_position+5]):
		if p[i][0] != card_value:
			return False
	return True,deck_position



def three_of_a_kind(p):
	deck_weights = get_deck_weights()
	first = p[0][0]
	second = p[1][0]
	third = p[2][0]
	values = [c[0] for c in p]
	if values.count(first) == 3:
		return True,deck_weights[first]
	elif values.count(second) == 3:
		return True,deck_weights[second]
	elif values.count(third) == 3:
		return True,deck_weights[third]
	else:
		return False

def two_pairs(p):
	deck_weights = get_deck_weights()
	high_pair = 0
	for _ in range(2):
		if one_pair(p):
			value = one_pair(p)[1]
			for _ in range(2):
				for c in p:
					if c[0] == str(value):
						card = c
						p.remove(card)
						high_pair = max(high_pair, deck_weights[card[0]])
						break
	if len(p) == 1:
		return True,high_pair
	else:
		return False



def one_pair(p):
	deck_weights = get_deck_weights()
	values = [c[0] for c in p]
	for value in values[:-1]:
		if values.count(value) == 2:
			return True,deck_weights[value]
	return False

def high_card(p):
	return p[-1]


def main():
	p1,p2 = readf('../additionals/p054_poker.txt')

	p1_score = 0
	for i in range(len(p1)):
		# We only need to keep track of p1
		if p1_best_hand(p1[i],p2[i]):
			p1_score += 1
	print(p1_score)


def get_deck_weights():
	deck_weights = {'2':2,'3':3,'4':4,'5':5,'6':6,'7':7,'8':8,'9':9,'T':10,'J':11,'Q':12,'K':13,'A':14}
	return deck_weights


def sort_cards(p):
	deck_weights = get_deck_weights()
	return sorted(p, key=lambda x: deck_weights[x[0]])


main()