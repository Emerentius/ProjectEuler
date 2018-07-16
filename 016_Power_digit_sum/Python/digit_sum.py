#!/usr/bin/env python3
number = 2**1000
string = str(number)

digit_sum = 0;
for char in string:
        digit_sum += int(char)

print(digit_sum)
