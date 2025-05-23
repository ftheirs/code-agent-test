def fibonacci(n):
    if n <= 1:
        return n
    else:
        return fibonacci(n-1) + fibonacci(n-2)

# Get input from the user
index = int(input("Enter an index between 0 and 30: "))

# Calculate the Fibonacci number
fib_number = fibonacci(index)

# Print the result
print("The Fibonacci number at index", index, "is", fib_number)
