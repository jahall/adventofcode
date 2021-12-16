import itertools
import functools
from pathlib import Path
from queue import PriorityQueue
import random
import sys

import numpy as np

sys.setrecursionlimit(10000)

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
  """Solve using value iteration...massive overkill.
  
  Also, useful learning: setting the right discount is important! Needs
  to be high enough to bump it out of short-term thinking!
  """
  def __init__(
    self,
    risks,
    discount=0.999999,
    max_iter=1000,
  ):
    self.risks = risks
    self.discount = discount
    self.max_iter = max_iter
    self.path = f"models/value-iteration-{risks.shape[0]}.npz"
    try:
      self.load()
    except FileNotFoundError:
      self.rewards = -risks.copy()
      self.value = np.zeros(self.risks.shape)
      for rc in self._iter_elements():
        self.value[rc] = self._init_value(rc)
      self.policy = np.empty(self.value.shape, dtype=object)
      self.policy[:] = "*"

  _transitions = {
    "*": lambda rc: rc,
    "<": lambda rc: (rc[0], rc[1] - 1),
    ">": lambda rc: (rc[0], rc[1] + 1),
    "^": lambda rc: (rc[0] - 1, rc[1]),
    "v": lambda rc: (rc[0] + 1, rc[1]),
  }

  @functools.cache
  def _init_value(self, rc):
    neighbour_values = []
    # constrain to move either right or down...
    for rc_n, _ in self._iter_actions(rc, ">v"):
      neighbour_value = self.rewards[rc_n] + self.discount * self._init_value(rc_n)
      neighbour_values.append(neighbour_value)
    return max(neighbour_values or [0])

  def solve(self):
    iteration = 0
    n_stable = 0
    while True:
      # 1. Perform value update
      iteration += 1
      n_changed = 0
      for rc in self._iter_elements(shuffle=True):
        is_final = rc == (self.risks.shape[0] - 1, self.risks.shape[1] - 1)
        neighbour_values = []
        actions = "<>v^" + ("*" if is_final else "")
        for rc_n, action in self._iter_actions(rc, actions):
          reward = self.rewards[rc_n]
          if action == "*" and is_final:
            reward = 0
          neighbour_value = reward + self.discount * self.value[rc_n]
          neighbour_values.append((neighbour_value, action))
        self.value[rc], best_action = max(neighbour_values)
        if best_action != self.policy[rc]:
          n_changed += 1
        self.policy[rc] = best_action
      # 2. Check for termination
      print(f"Iteration {iteration}: {n_changed} altered actions")
      n_stable = (n_stable + 1) if n_changed == 0 else 0
      if iteration == self.max_iter or n_stable == 10:
        break
      if not iteration % 10:
        self.save()
  
    print("\nActions in top 20x20:\n")
    for row in self.policy[:20,:20]:
      print("".join(2 * a + " " for a in row))
    print()
    
    print("Actions in bottom 20x20:\n")
    for row in self.policy[-20:,-20:]:
      print("".join(2 * a + " " for a in row))
    print()

    self.save()
    return int(np.ceil(-self.value[0,0]))

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

  def _iter_actions(self, rc, actions):
    for action in actions:
      rn, cn = self._transitions[action](rc)
      if 0 <= rn < self.risks.shape[0] and 0 <= cn < self.risks.shape[1]:
        yield (rn, cn), action

  def _iter_elements(self, shuffle=False):
    rows = range(self.rewards.shape[0])
    cols = range(self.rewards.shape[1])
    rcs = list(itertools.product(rows, cols))
    random.shuffle(rcs)
    return rcs


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
  solver = AsyncValueIterationSolver(_load_risks())
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
