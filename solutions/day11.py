from pathlib import Path

import numpy as np


def part_1():
  """Simple part 1"""
  grid = _get_grid()
  flashes = 0
  for _ in range(100):
    grid = _simulate_one_step(grid)
    flashes += (grid == 0).sum()
  print(f"PART 1: Total number of flashes is {flashes}")


def part_2():
  """Complex part 2"""
  grid = _get_grid()
  step = 0
  while True:
    step += 1
    grid = _simulate_one_step(grid)
    if grid.sum() == 0:
      break
  print(f"PART 2: All the octupi flash at step {step}")


def _simulate_one_step(grid):
  # 1. Add one
  grid += 1
  # 2. Do flashes
  flashed = set()
  while True:
    finished = True
    mask = np.zeros((10, 10))
    for r in range(10):
      for c in range(10):
        if grid[r, c] > 9 and (r, c) not in flashed:
          finished = False
          flashed.add((r, c))
          r_slice = slice(max(r - 1, 0), min(r + 2, 10))
          c_slice = slice(max(c - 1, 0), min(c + 2, 10))
          mask[r_slice, c_slice] += 1
    grid = grid + mask
    if finished:
      break
  # 3. Set flashed to zero
  grid = grid * (grid < 10)
  return grid


def _get_grid():
  input_file = Path(__file__).parent.parent / "data" / "day11.txt"
  with input_file.open() as f:
    grid = np.array([[int(c) for c in line.strip()] for line in f])
    return grid


if __name__ == "__main__":
  part_1()
  part_2()
