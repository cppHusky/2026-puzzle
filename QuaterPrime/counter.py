#!/bin/python
import time
import math
import os


def is_prime(num):
    if num < 2:
        return False
    if num == 2:
        return True
    if num % 2 == 0:
        return False
    for i in range(3, int(math.sqrt(num)) + 1, 2):
        if num % i == 0:
            return False
    return True


def find_prev_primes(from_num, count):
    primes = []
    num = from_num - 1
    while len(primes) < count and num > 1:
        if is_prime(num):
            primes.insert(0, num)
        num -= 1
    return primes


def find_next_primes(from_num, count):
    primes = []
    num = from_num + 1
    while len(primes) < count:
        if is_prime(num):
            primes.append(num)
        num += 1
    return primes


def format_time(seconds):
    hours = seconds // 3600
    minutes = (seconds % 3600) // 60
    secs = seconds % 60
    return f"{hours:02d}:{minutes:02d}:{secs:02d}"


def main():
    while True:
        now = int(time.time())
        prev_primes = find_prev_primes(now, 5)
        next_primes = find_next_primes(now, 5)

        os.system('cls' if os.name == 'nt' else 'clear')
        print(f"Current UNIX timestamp: {now}")
        
        print("\nPast 5 Prime UNIX Timestamps:")
        print("─" * 40)
        for i, prime in enumerate(prev_primes):
            seconds_ago = now - prime
            print(f"Prime {i + 1}: {prime} ({seconds_ago} seconds ago)")
        
        print("\nNext 5 Prime UNIX Timestamps:")
        print("─" * 40)
        for i, prime in enumerate(next_primes):
            seconds_until = prime - now
            print(f"Prime {i + 1}: {prime} (in {seconds_until} seconds)")

        print("─" * 40)
        print("Refreshing every second...")

        time.sleep(1)


if __name__ == "__main__":
    main()
