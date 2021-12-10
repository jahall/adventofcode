from collections import deque
from pathlib import Path


def part_1():
  """Simple part 1"""
  score = sum(_check_syntax(row) for row in _iter_rows())
  print(f"PART 1: Syntax error score is {score}")


def part_2():
  """Complex part 2"""
  rows = [row for row in _iter_rows() if _check_syntax(row) == 0]
  scores = [_calc_autocomplete_score(row) for row in rows]
  score = sorted(scores)[len(scores) // 2]
  print(f"PART 2: Autocomplete score is {score}")


def _check_syntax(row):
  stack = deque()
  close = {"(": ")", "[": "]", "{": "}", "<": ">"}
  scores = {")": 3, "]": 57, "}": 1197, ">": 25137}
  for c in row:
    if c in {"(", "[", "{", "<"}:
      stack.append(c)
    elif c == close[stack[-1]]:
      stack.pop()
    else:
      return scores[c]
  return 0


def _calc_autocomplete_score(row):
  stack = deque()
  # 1. Parse row
  for c in row:
    if c in {"(", "[", "{", "<"}:
      stack.append(c)
    else:  # assumes line is good
      stack.pop()
  # 2. Find completion
  score = 0
  scores = {"(": 1, "[": 2, "{": 3, "<": 4}
  while True:
    try:
      c = stack.pop()
      score = score * 5 + scores[c]
    except IndexError:
      break
  return score


def _iter_rows():
  input_file = Path(__file__).parent.parent / "data" / "day10.txt"
  with input_file.open() as f:
    for line in f:
      yield line.strip() 


if __name__ == "__main__":
  part_1()
  part_2()
