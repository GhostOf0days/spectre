import random
import string
import time

def measure_access_time(password, guess, index):
    start_time = time.perf_counter()
    # Speculative execution
    if password[index] == guess:
        temp = 0
        for _ in range(1000):
            temp += 1
    end_time = time.perf_counter()
    return end_time - start_time

def crack_password(password):
    characters = string.ascii_letters + string.digits + string.punctuation
    cracked_password = ['*'] * len(password)

    for i in range(len(password)):
        max_time = 0
        max_char = '*'
        for char in characters:
            total_time = 0
            for _ in range(100):
                access_time = measure_access_time(password, char, i)
                total_time += access_time
            avg_time = total_time / 100
            if avg_time > max_time:
                max_time = avg_time
                max_char = char
        cracked_password[i] = max_char

    return ''.join(cracked_password)

password ="hTQU?K4asW.xl,T\Gm?vj48`}"
print("Original Password:", password)

cracked_password = crack_password(password)
print("Cracked Password:", cracked_password)
