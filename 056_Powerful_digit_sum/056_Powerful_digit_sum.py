def digit_sum(string):
    summ = 0
    for ch in string:
        summ += int(ch)
    return summ

max_digit_sum = 0
for a in range(1,100):
    for b in range(1,100):
        digit_sum_ = digit_sum(str(a**b))
        if digit_sum_ > max_digit_sum:
            max_digit_sum = digit_sum_
print max_digit_sum
