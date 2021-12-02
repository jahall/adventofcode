from pathlib import Path

def part_1():
  """Simple part 1"""
  n = _do_calculation(_iter_depths())
  print(f"PART 1: Depths larger than previous: {n}")


def part_2():
  """Tricky part 2"""
  depths = list(_iter_depths())
  noise_free_depths = []
  for i in range(1, len(depths) - 1):
    noise_free_depths.append(depths[i - 1] + depths[i] + depths[i + 1])
  n = _do_calculation(noise_free_depths)
  print(f"PART 2: Depths larger than previous: {n}")


def _do_calculation(depths):
  n = 0
  prev_depth = None
  for depth in depths:
    if prev_depth is not None and depth > prev_depth:
      n += 1
    prev_depth = depth
  return n


def _iter_depths():
  input_file = Path(__file__).parent.parent / "data" / "day1.txt"
  with input_file.open() as f:
    for line in f:
      yield int(line.strip())


if __name__ == "__main__":
  part_1()
  part_2()