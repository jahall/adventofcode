import math
from pathlib import Path

import numpy as np
import scipy.optimize


def part_1():
  """Simple part 1"""
  positions = _get_positions()
  middle = int(np.median(positions).round())
  fuel_used = int(np.abs(positions - middle).sum())
  print(f"PART 1: Middle position is {middle} and total fuel is {fuel_used}")


def part_2_brute():
  """Complex part 2"""
  positions = _get_positions()
  best_fuel, best_middle = np.inf, 0
  for middle in range(positions.min(), positions.max() + 1):
    fuel = 0
    for pos in (positions - middle):
      fuel += np.arange(abs(pos) + 1).sum()
    if fuel < best_fuel:
      best_fuel, best_middle = fuel, middle
  print(f"PART 2: Middle position is {best_middle} and total fuel is {best_fuel}")


def part_2_optimize():
  """Complex part 2"""

  def calc_fuel(middle):
    fuel = 0
    for pos in (positions - middle):
      fuel += np.arange(abs(pos) + 1).sum()
    return fuel

  positions = _get_positions()
  solution = scipy.optimize.golden(calc_fuel, brack=(positions.min(), positions.max()))
  best_middles = math.floor(solution), math.ceil(solution)
  best_fuels = calc_fuel(best_middles[0]), calc_fuel(best_middles[1])
  idx = 0 if best_fuels[0] < best_fuels[1] else 1
  print(f"PART 2: Middle position is {best_middles[idx]} and total fuel is {best_fuels[idx]}")

 
def _get_positions():
  input_file = Path(__file__).parent.parent / "data" / "day7.txt"
  with input_file.open() as f:
    return np.array([int(i) for i in next(f).strip().split(",")])


if __name__ == "__main__":
  part_1()
  part_2_brute()
  part_2_optimize()