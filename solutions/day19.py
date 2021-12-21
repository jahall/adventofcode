from collections import defaultdict
import functools
import itertools
import math
from pathlib import Path

import numpy as np


def part_1():
  """Simple part 1"""
  data = _load_scanner_data()
  scanners = {0: (0, 0, 0)}
  scanner_beacons = {0: data.pop(0)}
  beacons = set(scanner_beacons[0])
  compared = set()
  while True:
    if not data:
      break
    for s1_id, b1 in data.items():
      found = False
      for s0_id, b0 in scanner_beacons.items():
        key = tuple(sorted([s0_id, s1_id]))
        if key not in compared:
          compared.add(key)
          print(f"Comparing scanner {s1_id} with re-mapped sensor {s0_id}")
          result = _attempt_translation(b0, b1)
          if result:
            found = True
            s1, b1 = result
            scanners[s1_id] = s1
            scanner_beacons[s1_id] = b1
            beacons.update(b1)
            data.pop(s1_id)
            print()
            print(f" - SCANNER {s1_id} LIVES AT {s1}")
            print(f" - Mapped {len(beacons)} beacons")
            print(f" - Only {len(data)} un-mapped scanners remaining!\n")
            break
      if found:
        break
  print(f"PART 1: There are {len(beacons)} beacons in total")
  return scanners


def part_2(scanners):
  """Complex part 2"""
  max_distance = 0
  for s1, s2 in itertools.combinations(scanners.values(), 2):
    x1, y1, z1 = s1
    x2, y2, z2 = s2
    dist = abs(x1 - x2) + abs(y1 - y2) + abs(z1 - z2)
    if dist > max_distance:
      max_distance = dist
  print(f"PART 2: The largest Manhatten distance is {max_distance}")


def _attempt_translation(b0, b1, k=12):
  """Attempt to translate b1.

  Do this by checking if there are sufficient number of common pair differences
  between the two sets of coordinates.
  """
  b0_pairs = _to_pairs(b0)
  necessary_matches = math.comb(k, 2)
  for rot in _rotation_matrices():
    b1_rot = [_rotate(b, rot) for b in b1]
    b1_pairs = _to_pairs(b1_rot)
    common = set(b0_pairs) & set(b1_pairs)
    b0_count = sum(len(pairs) for diff, pairs in b0_pairs.items() if diff in common)
    b1_count = sum(len(pairs) for diff, pairs in b1_pairs.items() if diff in common)
    if b0_count >= necessary_matches and b1_count >= necessary_matches:
      b0_beacon, b1_beacon = None, None
      for diff in common:
        if len(b0_pairs[diff]) == 1 and len(b1_pairs[diff]) == 1:
          b0_beacon = b0_pairs[diff][0][0]
          b1_beacon = b1_pairs[diff][0][0]
      if b0_beacon is None:
        raise RuntimeError("Couldn't find unambiguous common point")
      offset = _minus(b1_beacon, b0_beacon)
      s1 = _minus((0, 0, 0), offset)
      b1 = [_minus(b, offset) for b in b1_rot]
      return s1, b1


def _to_pairs(beacons):
  """Convert list of beacons to mapping from pair differences to list of associated pairs."""
  beacons = sorted(beacons)
  pairs = defaultdict(list)
  for p1, p2 in itertools.combinations(beacons, 2):
    pairs[_minus(p2, p1)].append((p1, p2))
  return pairs

  
def _minus(p1, p2):
  x1, y1, z1 = p1
  x2, y2, z2 = p2
  return x1 - x2, y1 - y2, z1 - z2


def _rotate(point, matrix):
  """Rotate point using a rotation matrix."""
  arr = np.array([[v] for v in point])
  x, y, z = (matrix @ arr).flatten()
  return int(x), int(y), int(z)


@functools.cache
def _rotation_matrices():
  # Load the 24 possible valid rotation matrices
  xrots = [
    np.array([[1, 0, 0], [0, +1, +0], [0, +0, +1]]),
    np.array([[1, 0, 0], [0, +0, -1], [0, +1, +0]]),
    np.array([[1, 0, 0], [0, -1, +0], [0, +0, -1]]),
    np.array([[1, 0, 0], [0, +0, +1], [0, -1, +0]]),
  ]
  yrots = [
    np.array([[+1, 0, +0], [0, 1, 0], [+0, 0, +1]]),
    np.array([[+0, 0, +1], [0, 1, 0], [-1, 0, +0]]),
    np.array([[-1, 0, +0], [0, 1, 0], [+0, 0, -1]]),
    np.array([[+0, 0, -1], [0, 1, 0], [+1, 0, +0]]),
  ]
  zrots = [
    np.array([[+1, +0, 0], [+0, +1, 0], [0, 0, 1]]),
    np.array([[+0, +1, 0], [-1, +0, 0], [0, 0, 1]]),
    np.array([[-1, +0, 0], [+0, -1, 0], [0, 0, 1]]),
    np.array([[+0, -1, 0], [+1, +0, 0], [0, 0, 1]]),
  ]
  all_rots = [
    (xrot @ yrot @ zrot).astype(int)
    for xrot, yrot, zrot in itertools.product(xrots, yrots, zrots)
  ]
  rots = []
  for rot in all_rots:
    if rots and any((rot == existing).all() for existing in rots):
      continue
    rots.append(rot)
  return rots


def _load_scanner_data(test=False):
  suffix = "_test" if test else ""
  input_file = Path(__file__).parent.parent / "data" / f"day19{suffix}.txt"
  scanner_data = []
  with input_file.open() as f:
    for line in f:
      if line.startswith("---"):
        scanner_data.append([])
      elif line.strip():
        point = tuple(int(i) for i in line.strip().split(","))
        scanner_data[-1].append(point)
  return {s_id: beacons for s_id, beacons in enumerate(scanner_data)}


if __name__ == "__main__":
  scanners = part_1()
  part_2(scanners)
