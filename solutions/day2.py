from pathlib import Path

def part_1():
  """Simple part 1"""
  x, depth = 0, 0
  for instruction, value in _iter_instructions():
    if instruction == "forward":
      x += value
    elif instruction == "down":
      depth += value
    elif instruction == "up":
      depth -= value
  print(f"PART 1: Location is: {x} x {depth} = {x * depth}")


def part_2():
  """Tricky part 2"""
  hor, depth, aim = 0, 0, 0
  for instruction, x in _iter_instructions():
    if instruction == "forward":
      hor += x
      depth += (aim * x)
    elif instruction == "down":
      aim += x
    elif instruction == "up":
      aim -= x
  print(f"PART 2: Location is: {hor} x {depth} = {hor * depth}")


def _iter_instructions():
  input_file = Path(__file__).parent.parent / "data" / "day2.txt"
  with input_file.open() as f:
    for line in f:
      instruction, value = line.strip().split()
      yield instruction, int(value)


if __name__ == "__main__":
  part_1()
  part_2()