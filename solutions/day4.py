from copy import deepcopy
from pathlib import Path

import numpy as np
from numpy.lib.twodim_base import diag

class Board:
  def __init__(self, nums, marked=None):
    self._nums = nums
    self._marked = marked if marked is not None else np.zeros((5, 5))

  def mark(self, num):
    self._marked += (self._nums == num)
    if (self._marked.sum(axis=0) == 5).any() or (self._marked.sum(axis=1) == 5).any():
      return True
    return False

  def score(self):
    return int((self._nums * (1 - self._marked)).sum())

  def copy(self):
    return Board(np.copy(self._nums), np.copy(self._marked))

  def __repr__(self):
    return f"{self._nums}\n{self._marked}" 

  __str__ = __repr__


def part_1():
  """Simple part 1"""
  draw_nums, boards = _parse_input()
  for num in draw_nums:
    for board in boards:
      complete = board.mark(num)
      if complete:
        score = board.score()
        print(f"PART 1: Final score is {score} * {num} = {score * num} from\n{board}")
        return


def part_2():
  """Complex part 2"""
  draw_nums, boards = _parse_input()
  completed_boards = []
  completed_indices = set()
  for num in draw_nums:
    for i, board in enumerate(boards):
      complete = board.mark(num)
      if complete and i not in completed_indices:
        completed_boards.append((board.copy(), num))
        completed_indices.add(i)
  board, num = completed_boards[-1]
  score = board.score()
  print(f"PART 2: Final score is {score} * {num} = {score * num} from\n{board}")
        

def _parse_input():
  input_file = Path(__file__).parent.parent / "data" / "day4.txt"
  with input_file.open() as f:
    draw_nums = [int(i) for i in next(f).strip().split(",")]
    boards = []
    while True:
      try:
        next(f)  # skip blank row
      except StopIteration:
        break
      nums = []
      for _ in range(5):
        nums.append([int(i) for i in next(f).strip().split()])
      boards.append(Board(np.array(nums)))
  return draw_nums, boards


if __name__ == "__main__":
  part_1()
  part_2()