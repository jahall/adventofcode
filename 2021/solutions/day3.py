from pathlib import Path

import numpy as np
from numpy.lib.twodim_base import diag

def part_1():
  """Simple part 1"""
  diagnostics = _get_diagnostics()
  threshold = len(diagnostics) / 2
  gamma, epsilon = [], []
  for col in diagnostics.T:
    most_freq = 0 if col.sum() < threshold else 1
    gamma.append(most_freq)
    epsilon.append(int(not most_freq))
  gamma = _to_value(gamma)
  epsilon = _to_value(epsilon)
  print(f"PART 1: Power consumption: {gamma} x {epsilon} = {gamma * epsilon}")


def part_2():
  """Tricky part 2"""
  ox_gen = _extract_rating(lambda col: 1 if col.sum() >= (len(col) / 2) else 0)
  co2_scrub = _extract_rating(lambda col: 0 if col.sum() >= (len(col) / 2) else 1)
  print(f"PART 2: Life support rating is: {ox_gen} x {co2_scrub} = {ox_gen * co2_scrub}")


def _extract_rating(condition):
  diagnostics = _get_diagnostics()
  while True:
    for bit_pos in range(diagnostics.shape[1]):
      col = diagnostics[:, bit_pos]
      diagnostics = diagnostics[col == condition(col)]
      if len(diagnostics) == 1:
        return _to_value(diagnostics[0,:])


def _to_value(num_list):
  val = 0
  for p, v in enumerate(reversed(num_list)):
    val += v * (2 ** p)
  return val


def _get_diagnostics():
  arr = []
  for row in _iter_rows():
    arr.append([int(bit) for bit in row])
  return np.array(arr)


def _iter_rows():
  input_file = Path(__file__).parent.parent / "data" / "day3.txt"
  with input_file.open() as f:
    for line in f:
      yield line.strip()


if __name__ == "__main__":
  part_1()
  part_2()