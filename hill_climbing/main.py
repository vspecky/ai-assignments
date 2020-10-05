import sys
from os import path
from random import random, shuffle
from math import floor
import matplotlib.pyplot as plt

USAGE = \
"""
Usage:
    python main.py <operator> <generations>

Where:
    <operator> can be:
        rev  - Reversing Operator
        swap - Swapping Operator

    <generations> is a positive number
"""

CITIES_INFO = \
"""
There should be a cities.tsp file in the directory of the file containing the cities table.

Example Table:
    A, 0, 1
    B, 0, 2
    C, 0, 3
    D, 0, 4

The format "NAME, X, Y" should be followed and one line should contain one city."
"""

def parse_cities(path):
    with open(path, 'r') as f:
        file_str = f.read().strip()

    rows = file_str.split('\n')

    if len(rows) < 3:
        raise Exception("There should be at least 3 cities")

    cities = []

    for row in rows:
        city_info = row.split(',')
        if len(city_info) != 3:
            raise Exception(f"Error parsing city info @ '{row}'")

        city_name = city_info[0].strip()
        city_x = float(city_info[1].strip())
        city_y = float(city_info[2].strip())

        cities.append({ 'name': city_name, 'x': city_x, 'y': city_y })

    if len(cities) < 3:
        raise Exception("There should be at least 3 UNIQUELY NAMED cities")

    return cities

class Operators:
    Reverse = 0
    Swap = 1

class Gene(object):
    def __init__(self, city_name, city_x, city_y):
        self.name = city_name
        self.x = city_x
        self.y = city_y

    def __add__(self, other):
        if not isinstance(other, Gene):
            raise Exception(f"Can't add a Gene to {type(other)}")

        return ((self.x - other.x) ** 2 + (self.y - other.y) ** 2) ** 0.5

    def __str__(self):
        return f"City({self.name}, ({self.x}, {self.y}))"

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        if not isinstance(other, Gene):
            return False

        return self.name == other.name

class Genome(object):
    def __init__(self, genes):
        self.genes = genes
        self.calculate_fitness()

    def calculate_fitness(self):
        self.fitness = 0

        for i in range(0, len(self.genes) - 1):
            self.fitness += (self.genes[i] + self.genes[i + 1])

        self.fitness += (self.genes[0] + self.genes[len(self.genes) - 1])

    def plot(self):
        x = []
        y = []
        names = []
        for g in self.genes:
            x.append(g.x)
            y.append(g.y)
            names.append(g.name)
        
        plt.plot([*x, x[0]], [*y, y[0]], 'o-k', lw=1, ms=2)
        for i in range(len(names)):
            plt.annotate(names[i], (x[i], y[i]))

        plt.show()

    def __str__(self):
        return f"Genome {{\n    fitness: {self.fitness},\n    path: {' -> '.join([g.name for g in self.genes]) + ' -> ' + self.genes[0].name}\n}}"

    def __eq__(self, other):
        if not isinstance(other, Genome):
            return False

        for i in range(len(self.genes)):
            if self.genes[i] != other.genes[i]:
                return False

        return True


class TSP_HC(object):
    def __init__(self, file_path, operator):
        self.cities = parse_cities(file_path)
        self.genes = [Gene(city['name'], city['x'], city['y']) for city in self.cities]
        self.best_genome = None
        self.bestest_genome = None
        self.bestest_fitness = float('inf')
        self.best_fitness = float('inf')
        self.operator = operator

    def init_best_genome(self):
        gene_pool = self.genes[1:]
        origin = self.genes[0]

        shuffle(gene_pool)
        self.best_genome = Genome([origin, *gene_pool])
        self.bestest_genome = self.best_genome
        self.best_fitness = self.best_genome.fitness
        self.bestest_fitness = self.best_fitness
        print(self.best_genome)

    def init_gen(self):
        gene_pool = self.genes[1:]
        origin = self.genes[0]

        shuffle(gene_pool)
        self.best_genome = Genome([origin, *gene_pool])
        self.best_fitness = self.best_genome.fitness

    def reverse(self):
        best_genes = self.best_genome.genes[1:]
        best_genome = self.best_genome
        for i in range(len(best_genes) - 1):
            for j in range(i + 1, len(best_genes)):
                genes = best_genes[:]    
                genes[i:j] = list(reversed(genes[i:j]))
                genome = Genome([self.genes[0], *genes])
                if genome.fitness < best_genome.fitness:
                    best_genome = genome

        return best_genome

    def swap(self):
        best_genes = self.best_genome.genes[1:]
        best_genome = self.best_genome
        for i in range(len(best_genes) - 1):
            for j in range(i + 1, len(best_genes)):
                genes = best_genes[:]
                genes[i], genes[j] = genes[j], genes[i]
                genome = Genome([self.genes[0], *genes])
                if genome.fitness < best_genome.fitness:
                    best_genome = genome

        return best_genome

    def progenate(self):
        best_genome = self.swap() if self.operator == Operators.Swap else self.reverse()

        if best_genome == self.best_genome:
            return False

        self.best_genome = best_genome

        if best_genome.fitness < self.bestest_genome.fitness:
            self.bestest_genome = best_genome

        return True

    def commence(self, generations=100):
        self.init_best_genome()

        for gen in range(generations):
            self.init_gen()

            while True:
                if not self.progenate():
                    break

            print("======================================================")
            print(f"Generation: {gen + 1}")
            print("Best Genome:")
            print(str(self.bestest_genome))
            print("======================================================")

    
def main():
    dir_path = path.dirname(__file__)
    cities_path = path.join(dir_path, "cities.tsp")

    if len(sys.argv) != 3:
        print(USAGE)

    operator = sys.argv[1]
    if operator not in ['swap', 'rev']:
        print(USAGE)

    generations = int(sys.argv[2])

    if generations < 1:
        print(USAGE)

    hc = TSP_HC(cities_path, operator)
    hc.commence(generations)
    hc.bestest_genome.plot()

if __name__ == "__main__":
    main()

