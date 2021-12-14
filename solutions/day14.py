from collections import Counter, defaultdict
from pathlib import Path


class SimplePolymer:
  def __init__(self, sequence, insertions) -> None:
    self.sequence = sequence
    self.insertions = insertions

  def step(self) -> None:
    """Run one step of the insertion process."""
    for i in reversed(range(len(self.sequence) - 1)):
      pair = self.sequence[i:i + 2]
      to_insert = self.insertions.get(pair)
      if to_insert:
        self.sequence = self.sequence[:i + 1] + to_insert + self.sequence[i + 1:]

  def score(self) -> float:
    """Score the sequence."""
    counts = Counter(self.sequence)
    mc = counts.most_common()
    return mc[0][1] - mc[-1][1]


class CleverPolymer:
  def __init__(self, sequence, insertions) -> None:
    self.start = sequence[0]
    self.end = sequence[-1]
    self.pair_counts = defaultdict(int)
    for i in range(len(sequence) - 1):
      self.pair_counts[sequence[i: i + 2]] += 1
    self.insertions = insertions

  def step(self) -> None:
    """Run one step of the insertion process."""
    for pair, count in list(self.pair_counts.items()):
      to_insert = self.insertions.get(pair)
      if to_insert:
        self.pair_counts[pair] -= count
        self.pair_counts[pair[0] + to_insert] += count
        self.pair_counts[to_insert + pair[1]] += count

  def score(self) -> float:
    """Score the sequence."""
    char_counts = defaultdict(int)
    for pair, count in self.pair_counts.items():
      char_counts[pair[0]] += count
      char_counts[pair[1]] += count
    char_counts[self.start] += 1
    char_counts[self.end] += 1
    char_counts = {char: count // 2 for char, count in char_counts.items()}
    char_counts = sorted(char_counts.values())
    return char_counts[-1] - char_counts[0]


def part_1():
  """Simple part 1"""
  polymer = SimplePolymer(*_load_problem())
  for _ in range(10):
    polymer.step()
  score = polymer.score()
  print(f"PART 1: After 10 steps the score is: {score}")


def part_2():
  """Complex part 2"""
  polymer = CleverPolymer(*_load_problem())
  for _ in range(40):
    polymer.step()
  score = polymer.score()
  print(f"PART 1: After 40 steps the score is: {score}")


def _load_problem():
  input_file = Path(__file__).parent.parent / "data" / "day14.txt"
  with input_file.open() as f:
    sequence = next(f).strip()
    insertions = {}
    next(f)
    for line in f:
      pair, _, elem = line.strip().split()
      insertions[pair] = elem
  return sequence, insertions


if __name__ == "__main__":
  part_1()
  part_2()
