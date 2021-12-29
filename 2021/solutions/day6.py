from collections import Counter
import functools
import math
from pathlib import Path


def part_1():
  """Simple part 1"""
  n = _solve_challenge(days=80)
  print(f"PART 1: After 80 days there are {n} lanternfish")


def part_2():
  """Complex part 2"""
  n = _solve_challenge(days=256)
  print(f"PART 2: After 256 days there are {n} lanternfish")


def _solve_challenge(days):
  states = _init_fish()
  n = len(states)
  for state, po in Counter(states).items():
    n += po * _simulate_fish(days - state - 1)
  return n


@functools.cache
def _simulate_fish(t):
  n_children = math.floor(t / 7 + 1)
  for _ in range(1, n_children + 1):
    if t - 9 < 0:
      break
    n_children += _simulate_fish(t - 9)
    t -= 7
  return n_children
        
 
def _init_fish():
  input_file = Path(__file__).parent.parent / "data" / "day6.txt"
  with input_file.open() as f:
    return [int(i) for i in next(f).strip().split(",")]


if __name__ == "__main__":
  part_1()
  part_2()