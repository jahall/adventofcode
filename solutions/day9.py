from pathlib import Path

import numpy as np


def part_1():
  """Simple part 1"""
  grid = _load_grid()
  risk = sum(height + 1 for _, height in _iter_low_points(grid))
  print(f"PART 1: Total risk is {risk}")


def part_2():
  """Complex part 2"""
  grid = _load_grid()
  basin_sizes = [_size_of_basin(grid, p) for p, _ in _iter_low_points(grid)]
  size = np.prod(sorted(basin_sizes)[-3:])
  print(f"PART 2: Total size of biggest 3 basins is {size}")


def _size_of_basin(grid, low_point):
  basin = {low_point}
  while True:
    finished = True
    for point in list(basin):
      for neighbour, height in _iter_neighbours(grid, point):
        if neighbour not in basin and height < 9:
          finished = False
          basin.add(neighbour)
    if finished:
      break
  return len(basin)


def _iter_low_points(grid):
  for r in range(1, grid.shape[0] - 1):
    for c in range(1, grid.shape[1] - 1):
      height = grid[r,c]
      if all(height < n_height for _, n_height in _iter_neighbours(grid, (r, c))):
        yield (r, c), height


def _iter_neighbours(grid, point):
  r, c = point
  for p in [(r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c)]:
    yield p, grid[p[0], p[1]]


def _load_grid():
  # include padding rows/columns to make things easier down the line
  input_file = Path(__file__).parent.parent / "data" / "day9.txt"
  with input_file.open() as f:
    grid = []
    for line in f:
      grid.append([9] + [int(i) for i in line.strip()] + [9])
  ncols = len(grid[0])
  grid = [[9] * ncols] + grid + [[9] * ncols]
  return np.array(grid)


if __name__ == "__main__":
  part_1()
  part_2()
