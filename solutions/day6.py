from collections import Counter
import math
from pathlib import Path


class Lanternfish:
  def __init__(self, state=8):
    self._state = state

  def update(self):
    if self._state == 0:
      self._state = 6
      return True
    else:
      self._state -= 1
      return False


def part_1():
  """Simple part 1"""
  fish_list = [Lanternfish(i) for i in _init_fish()]
  fish_list = _simulate_days(fish_list, days=80)
  print(f"PART 1: After 80 days there are {len(fish_list)} lanternfish")


def part_2():
  """Complex part 2"""
  offsets = _init_fish()
  n = 0
  for offset, po in Counter(offsets).items():
    print(offset, po)
    n += _simulate_fish(256, po, offset)
  print(f"PART 2: After 256 days there are {n} lanternfish")


def _simulate_fish(t, po, offset=8):
  if t < offset:
    return 0
  n_children = po * math.floor((t - offset) / 6 + 1)
  n_more = sum(_simulate_fish(t, po, offset=offset + 8 * i) for i in range(1, n_children + 1))
  return n_children + n_more
  

def _simulate_days(fish_list, days):
  for _ in range(days):
    for fish in list(fish_list):
      if fish.update():
        fish_list.append(Lanternfish())
  return fish_list
        
 
def _init_fish():
  input_file = Path(__file__).parent.parent / "data" / "day6.txt"
  with input_file.open() as f:
    return [int(i) for i in next(f).strip().split(",")]


if __name__ == "__main__":
  part_1()
  part_2()