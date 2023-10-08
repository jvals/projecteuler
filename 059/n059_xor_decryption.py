'''
Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.

A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value, taken from a secret key. The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.

For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes. The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.

Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key. If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message. The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.

Your task has been made easy, as the encryption key consists of three lower case characters. Using cipher.txt (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the original text.
'''


with open('additionals/p059_cipher.txt') as doc:
	ciphers = doc.read().strip().split(',')


def main():
	keys = 'abcdefghijklmnopqrstuvwxyz'
	for a in keys:
		for b in keys:
			for c in keys:
				key = a+b+c # key is now on form aaa, aab, ..., zzz
				l = len(key)
				# This next line iterates through the cipher text and creates a new list with decrypted characters..
				# ord() creates an ascii code from a char, and chr() creates a char from an ascii code.
				# ^ is a bitwise operator, namely XOR
				# key[i % l] cycles through the key, as i is an ever increasing value
				decrypted_ciphers = [chr(int(x) ^ ord(key[i % l])) for i,x in enumerate(ciphers)]
				# "".join() makes a nice string from a list
				decrypted_ciphers = "".join(decrypted_ciphers)
				# we check if "the" is in the deciphered text, if not it's probably gibberish
				if ' the ' in decrypted_ciphers: 
					print(sum(ord(s) for s in decrypted_ciphers)) # print the sum of the ascii codes
					print(key,decrypted_ciphers)

main()