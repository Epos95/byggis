# main.py for problem: hangman
from sys import stdin


string = stdin.readline().replace("\n", "")
alphabet = stdin.readline().replace("\n", "")

s = []
for char in string:
    if char not in s:
        s.append(char)

counter = 0
for char in alphabet:
    if len(s) == 0:
        print("WIN")
        break
    elif char in s:
        s.remove(char)
    else:
        counter += 1
    
    if counter == 10:
        print("LOSE")
        break
    
