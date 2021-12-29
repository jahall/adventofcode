from collections import defaultdict
from pathlib import Path


class Solver:
  def __init__(self, graph, allow_twice=False):
    self._graph = graph
    self._allow_twice = allow_twice

  def solve(self, root_path=None, visited_twice=False):
    """Find all possible paths from this root."""
    paths = []
    root_path = root_path or ["start"]
    if root_path[-1] != "end":
      for c in self._graph[root_path[-1]]:
        new_root = root_path + [c]
        if c == "start":
          continue
        elif c.isupper() or (c.islower() and c not in root_path) or c == "end":
          paths.append(new_root)
          paths.extend(self.solve(new_root, visited_twice=visited_twice))
        elif c.islower() and self._allow_twice and not visited_twice:
          paths.append(new_root)
          paths.extend(self.solve(new_root, visited_twice=True))
    return paths


def part_1():
  """Simple part 1"""
  graph = _get_graph()
  paths = Solver(graph).solve()
  paths = [p for p in paths if p[-1] == "end"]
  print(f"PART 1: Total paths through the caves is {len(paths)}")


def part_2():
  """Complex part 2"""
  graph = _get_graph()
  paths = Solver(graph, allow_twice=True).solve()
  paths = [p for p in paths if p[-1] == "end"]
  print(f"PART 2: Total paths through the caves is {len(paths)}")


def _get_graph():
  input_file = Path(__file__).parent.parent / "data" / "day12.txt"
  graph = defaultdict(list)
  with input_file.open() as f:
    for line in f:
      c1, c2 = line.strip().split("-")
      graph[c1].append(c2)
      graph[c2].append(c1)
  return graph


if __name__ == "__main__":
  part_1()
  part_2()
