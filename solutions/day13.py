from pathlib import Path


def part_1():
  """Simple part 1"""
  grid, folds = _load_problem()
  grid = _apply_fold(grid, folds[0])
  print(f"PART 1: After the first fold there are {len(grid)} dots")


def part_2():
  """Complex part 2"""
  grid, folds = _load_problem()
  for fold in folds:
    grid = _apply_fold(grid, fold)
  xmax = max(x for x, _ in grid)
  ymax = max(y for _, y in grid)
  print(f"PART 2: The hidden message is as follows:")
  print()
  for y in range(ymax + 1):
    row = ["##" if (x, y) in grid else "  " for x in range(xmax + 1)]
    print("".join(row))
  print()


def _apply_fold(grid, fold):
  new_grid = set()
  dim, val = fold
  for (x, y) in grid:
    if dim == "x" and x > val:
      x = 2 * val - x
    elif dim == "y" and y > val:
      y = 2 * val - y
    new_grid.add((x, y))
  return new_grid
      


def _load_problem():
  input_file = Path(__file__).parent.parent / "data" / "day13.txt"
  grid, folds = set(), []
  with input_file.open() as f:
    load_grid = True
    for line in f:
      line = line.strip()
      if not line:
        load_grid = False
      elif load_grid:
        grid.add(tuple(int(i) for i in line.split(",")))
      else:
        fold = line.split()[-1]
        dim, val = fold.split("=")
        folds.append((dim, int(val)))
  return grid, folds


if __name__ == "__main__":
  part_1()
  part_2()
