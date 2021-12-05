from collections import defaultdict
from pathlib import Path


def part_1():
  """Simple part 1"""
  num_dangerous = _solve_problem(include_diag=False)
  print(f"PART 1: Num dangerous points are {num_dangerous}")


def part_2():
  """Complext part 2"""
  num_dangerous = _solve_problem(include_diag=True)
  print(f"PART 2: Num dangerous points are {num_dangerous}")


def _solve_problem(include_diag=False):
  points = defaultdict(int)
  for (x1, y1), (x2, y2) in _iter_points():
    # Vertical line
    if x1 == x2:
      for y in range(y1, y2 + 1):
        points[(x1, y)] += 1
    # Horizontal line
    elif y1 == y2:
      for x in range(x1, x2 + 1):
        points[(x, y1)] += 1
    # Diagonal line
    elif include_diag:
      x_range = range(x1, x2 + 1)
      y_range = range(y1, y2 + 1) if y1 <= y2 else reversed(range(y2, y1 + 1))
      for x, y in zip(x_range, y_range):
        points[(x, y)] += 1
  return len([p for p, count in points.items() if count >= 2])
        

def _iter_points():
  input_file = Path(__file__).parent.parent / "data" / "day5.txt"
  with input_file.open() as f:
    for line in f:
      pair1, _, pair2 = line.strip().split()
      x1, y1 = pair1.split(",")
      x2, y2 = pair2.split(",")
      # ensure left-most and lower-most point is first
      p1, p2 = sorted([(int(x1), int(y1)), (int(x2), int(y2))])
      yield p1, p2


if __name__ == "__main__":
  part_1()
  part_2()