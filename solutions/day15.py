import functools
from pathlib import Path
from queue import PriorityQueue
import random

import numpy as np


class RecursiveSolver:
  """Solve recursively...but constrained to moving right or down."""
  def __init__(self, risks):
    self.risks = risks

  @functools.cache
  def solve(self, rc, include_self=False):
    next_risks = []
    r, c = rc
    nrows, ncols = self.risks.shape
    # constrained to move either right or down...
    for ro, co in [(r, c + 1), (r + 1, c)]:
      if 0 <= ro < nrows and 0 <= co < ncols:
        next_risks.append(self.solve((ro, co), include_self=True))
    self_risk = self.risks[r, c] if include_self else 0
    next_optimal_risk = min(next_risks or [0])
    return self_risk + next_optimal_risk


class DjikstraSolver:
  """Solve using Djikstra...the right way!"""
  def __init__(self, risks):
    self.risks = risks

  def solve(self, start=(0, 0)):
    pq = PriorityQueue()
    pq.put((0, start))

    costs = np.inf * np.ones(self.risks.shape)
    costs[start] = 0

    visited = set()
    while not pq.empty():
      _, vertex = pq.get()  # pops vertex with smallest distance
      visited.add(vertex)

      for neighbour, risk in self._iter_neighbours(vertex):
        if neighbour not in visited:
          old_cost = costs[neighbour]
          new_cost = costs[vertex] + risk
          if new_cost < old_cost:
            pq.put((new_cost, neighbour))
            costs[neighbour] = new_cost

    return int(costs[-1, -1])

  def _iter_neighbours(self, vertex):
    r, c = vertex
    nrows, ncols = self.risks.shape
    for rn, cn in [(r, c - 1), (r, c + 1), (r - 1, c), (r + 1, c)]:
        if 0 <= rn < nrows and 0 <= cn < ncols:
          yield (rn, cn), self.risks[rn, cn]


class AsyncValueIterationSolver:
  """Solve using asynchronous value iteration...massive overkill."""
  def __init__(
    self,
    risks,
    max_iter=100,
    discount=0.99,
    final_reward=100_000,
    iterate=True,
    shuffle=False,
  ):
    self.risks = risks
    self.max_iter = max_iter
    self.discount = discount
    self.shuffle = shuffle
    self.path = f"models/async-value-iteration-{risks.shape[0]}.npz"
    try:
      self.load()
    except FileNotFoundError:
      self.value = np.zeros(self.risks.shape)
      self.policy = np.empty(self.risks.shape, dtype=object)
      self.policy[:] = "*"
      self.rewards = -risks.copy()
    self.rewards[-1, -1] = final_reward  # final state should have big reward

  def solve(self):
    nrows, ncols = self.risks.shape
    iteration, n_stable = 0, 0
    row_range, col_range = list(range(nrows)), list(range(ncols))
    if self.shuffle:
      random.shuffle(row_range)
      random.shuffle(col_range)
    while True:
      # 1. Perform value update
      if not self.max_iter:
        break
      iteration += 1
      n_changed, n_changed_first_100, n_changed_first_10000 = 0, 0, 0
      for r in row_range:
        for c in col_range:
          if r == nrows - 1 and c == ncols - 1:
            # if in final state...just stay there
            continue
          possibilities = []
          for action in ["*", "<", ">", "^", "v"]:
            ro, co = self._transitions[action](r, c)
            if 0 <= ro < nrows and 0 <= co < ncols:
              value = self.rewards[ro, co] + self.discount * self.value[ro, co]
              possibilities.append((value, action))
          best_value, best_action = max(possibilities)
          if best_action != self.policy[r, c]:
            n_changed += 1
            if r < 10 and c < 10:
              n_changed_first_100 += 1
            if r < 100 and c < 100:
              n_changed_first_10000 += 1
          self.value[r, c] = best_value
          self.policy[r, c] = best_action
      # 2. Check for termination
      print(
        f"Iteration {iteration}: Changed {n_changed} actions "
        f"({n_changed_first_100} in top 10x10) "
        f"({n_changed_first_10000} in top 100x100) ")
      n_stable = (n_stable + 1) if n_changed == 0 else 0
      if n_stable == 3 or iteration == self.max_iter:
        print("Saving...")
        self.save()
        break
      if not iteration % 10:
        print("Saving...")
        self.save()

    print("First 10x10:")
    print(self.policy[:10,:10])
    print(self.rewards[:10,:10])

    print("Last 10x10:")
    print(self.policy[-10:,-10:])
    print(self.rewards[-10:,-10:])

    return self.score()

  _transitions = {
    "*": lambda r, c: (r, c),  # stay
    "<": lambda r, c: (r, c - 1),  # left
    "^": lambda r, c: (r - 1, c),  # up
    ">": lambda r, c: (r, c + 1),  # right
    "v": lambda r, c: (r + 1, c),  # down
  }

  def score(self):
    score = 0
    r, c = 0, 0
    nrows, ncols = self.risks.shape
    visited = {(r, c)}
    while True:
      r, c = self._transitions[self.policy[r, c]](r, c)
      score += self.risks[r, c]
      if r == nrows - 1 and c == ncols - 1:
        break
      if (r, c) in visited:
        raise RuntimeError(f"Oops, already visited ({r}, {c})")
      visited.add((r, c))
    return score

  def save(self):
    np.savez(
      self.path,
      risks=self.risks,
      value=self.value,
      policy=self.policy,
      rewards=self.rewards,
    )

  def load(self):
    f = np.load(self.path, allow_pickle=True)
    self.risks = f["risks"]
    self.value = f["value"]
    self.policy = f["policy"]
    self.rewards = f["rewards"]


def part_1():
  """Simple part 1"""
  solver = RecursiveSolver(_load_risks())
  total_risk = solver.solve((0, 0))
  print(f"PART 1: The optimal total risk is: {total_risk}")


def part_2_djikstra():
  """Complex part 2"""
  solver = DjikstraSolver(_load_big_risks())
  total_risk = solver.solve((0, 0))
  print(f"PART 2: The optimal total risk is: {total_risk}")


def part_2_value_iteration():
  """Complex part 2"""
  solver = AsyncValueIterationSolver(_load_big_risks(), max_iter=1000)
  total_risk = solver.solve()
  print(f"PART 2: The optimal total risk is: {total_risk}")


def _load_big_risks():
  risks = _load_risks()
  blocks = [risks]
  for _ in range(8):
    block = blocks[-1] + 1
    block = np.where(block == 10, 1, block)
    blocks.append(block)
  return np.block([
    blocks[0:5],
    blocks[1:6],
    blocks[2:7],
    blocks[3:8],
    blocks[4:9],
  ])


def _load_risks():
  input_file = Path(__file__).parent.parent / "data" / "day15.txt"
  with input_file.open() as f:
    risks = []
    for line in f:
      risks.append([int(i) for i in line.strip()])
  return np.array(risks)


if __name__ == "__main__":
  part_1()
  part_2_djikstra()
  part_2_value_iteration()
