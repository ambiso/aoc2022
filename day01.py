#!/bin/env python3

with open('inputs/day01a') as f:
	lines = f.read().strip()

l = list(map(str.split, lines.split('\n\n')))
sums = list()
for sublist in l: 
	sublist = map(int, sublist)
	sums.append(sum(sublist))

sums.sort(reverse=True)
print(sum(sums[:3]))

#print([sum(x) for x in ])
