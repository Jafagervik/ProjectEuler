
import itertools

p = itertools.product([i for i in range(1, 100+1)])

print(p)

s = sum([int(c) for c in str(p)])

print(f"{s=}")
