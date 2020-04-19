import unittest
import n054_poker_hands as poker

class pokerHandTests(unittest.TestCase):

	''' It's safe to assume every hand is sorted.'''

	def test_royal_flush(self):
		self.assertTrue(poker.royal_flush(['TH','JH','QH','KH','AH']))
		self.assertFalse(poker.royal_flush(['TH','JH','QH','KH','AC']))
		self.assertFalse(poker.royal_flush(['TC','TH','JH','QH','KH']))


	def test_straight_flush(self):
		self.assertTrue(poker.straight_flush(['9S','TS','JS','QS','KS']))
		self.assertTrue(poker.straight_flush(['TC','JC','QC','KC','AC']))

		self.assertFalse(poker.straight_flush(['9S','TS','JC','QS','KS']))
		self.assertFalse(poker.straight_flush(['8S','TS','JS','QS','KS']))
		self.assertFalse(poker.straight_flush(['TS','TS','JS','QS','KS']))

		self.assertTrue(
			poker.straight_flush(['3S','4S','5S','6S','7S'])
			>
			poker.straight_flush(['2S','3S','4S','5S','6S']))

	def test_four_of_a_kind(self):
		self.assertTrue(poker.four_of_a_kind(['3C','3H','3D','3S','AC']))
		self.assertTrue(poker.four_of_a_kind(['TC','TH','TD','TS','4H']))

		self.assertFalse(poker.four_of_a_kind(['4C','4H','5C','5H','AH']))

		self.assertTrue(
			poker.four_of_a_kind(['QC','QS','QH','QD','5S'])
			>
			poker.four_of_a_kind(['4S','4C','4D','4H','AH']))


	def test_full_house(self):
		self.assertTrue(poker.full_house(['6C','6D','6H','JH','JS']))

		self.assertFalse(poker.full_house(['6C','6D','6H','QH','JS']))

	def test_flush(self):
		self.assertTrue(poker.flush(['2S','4S','6S','8S','TS']))

	def test_straight(self):
		self.assertTrue(poker.straight(['6H','7C','8H','9D','TS']))

		self.assertFalse(poker.straight(['9H','TC','TH','QD','KS']))

		self.assertTrue(
			poker.straight(['8H','9C','TH','JD','QS'])
			>
			poker.straight(['6H','7C','8H','9D','TS']))

	def test_three_of_a_kind(self):
		self.assertTrue(poker.three_of_a_kind(['9S','9D','2S','3C','9C']))
		self.assertTrue(poker.three_of_a_kind(['AS','AD','AC','3C','9C']))

		self.assertTrue(
			poker.three_of_a_kind(['5D','5C','5S','2D','7D'])
			>
			poker.three_of_a_kind(['3D','3C','3H','QS','KD'])

			)

	def test_two_pairs(self):
		self.assertTrue(poker.two_pairs(['2S','2H','4D','4C','TD']))
		self.assertFalse(poker.two_pairs(['2S','2H','4D','3C','TD']))

	def test_one_pair(self):
		self.assertTrue(poker.one_pair(['2S','4C','QH','9C','QC']))

	def test_sort(self):
		self.assertEqual(
			poker.sort_cards(['4S','TC','2H','AS','3D']) ,
			['2H','3D','4S','TC','AS'] )

unittest.main()