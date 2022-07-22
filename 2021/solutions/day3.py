from collections import Counter
from pathlib import Path


def part_1():
  """Simple part 1"""
  diagnostics, nbits = _get_diagnostics()
  gamma, epsilon = "", ""
  for bit in range(nbits):
    counts = Counter([row[bit] for row in diagnostics])
    if counts.get("1", 0) >= counts.get("0", 0):
      gamma += "1"
      epsilon += "0"
    else:
      gamma += "0"
      epsilon += "1"
  gamma = int(gamma, 2)  # thank you Johnny Kerr!
  epsilon = int(epsilon, 2)
  print(f"PART 1: Power consumption: {gamma} x {epsilon} = {gamma * epsilon}")


def part_2():
  """Tricky part 2"""
  ox_gen = _extract_rating(get_filter_value=_get_most_common)
  co2_scrub = _extract_rating(get_filter_value=_get_least_common)
  print(f"PART 2: Life support rating is: {ox_gen} x {co2_scrub} = {ox_gen * co2_scrub}")


def _extract_rating(get_filter_value):
  diagnostics, nbits = _get_diagnostics()
  while True:
    for bit in range(nbits):
      counts = Counter([row[bit] for row in diagnostics])
      filter_value = get_filter_value(counts)
      diagnostics = [row for row in diagnostics if row[bit] == filter_value]
      if len(diagnostics) == 1:
        return int(diagnostics[0], 2)


def _get_most_common(counts):
    if counts.get("1", 0) >= counts.get("0", 0):
      return "1"
    else:
      return "0"


def _get_least_common(counts):
    if counts.get("1", 0) < counts.get("0", 0):
      return "1"
    else:
      return "0"


def _get_diagnostics():
  input_file = Path(__file__).parent.parent / "data" / "day3.txt"
  with input_file.open() as f:
    diagnostics = [line.strip() for line in f]
    # extract nbits for convenience
    nbits = len(diagnostics[0])
    return diagnostics, nbits


if __name__ == "__main__":
  part_1()
  part_2()