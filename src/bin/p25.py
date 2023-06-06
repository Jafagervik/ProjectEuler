#!/usr/bin/python3

a = 1
b = 1
count = 2

while (True):
    a, b = b, a+b
    count += 1

    if len(str(b)) == 1000:
        break

print(count)
