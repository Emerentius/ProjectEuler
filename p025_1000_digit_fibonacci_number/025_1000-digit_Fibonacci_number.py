Fibonacci_numbers = [1,1]

i = 0
while True:
        Fibonacci_numbers += [Fibonacci_numbers[i]+Fibonacci_numbers[i+1]]
        i += 1
        if len(str(Fibonacci_numbers[-1])) >= 1000:
                break
print(i+2)
