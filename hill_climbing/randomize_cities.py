from random import random

cities = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"

with open('cities.tsp', 'w') as f:
    for city in cities:
        f.write(f"{city}, {random() * 100}, {random() * 100}\n")
