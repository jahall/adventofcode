import json
import functools
import itertools
from pathlib import Path

from joblib import Parallel, delayed
import numpy as np


_CACHE = Path("models/day19-cache.json")


def part_1():
  """Simple part 1"""
  data, remapped, compared = _load()
  all_beacons = set.union(*[set(x["beacons"]) for x in remapped if x])
  while True:
    if all(b is None for b in data):
      break
    for i, b1 in enumerate(data):
      if b1 is None:
        continue
      found = False
      for j, b0 in enumerate(remapped):
        if b0 is None:
          continue
        b0 = b0["beacons"]
        key = tuple(sorted([i, j]))
        if key not in compared:
          compared.add(key)
          print(f"Comparing scanner {i} with re-mapped sensor {j}")
          result = _find_overlap(b0, b1)
          if result:
            s1, b1 = result
            all_beacons.update(b1)
            print()
            print(f" - SCANNER {i} LIVES AT {s1}")
            print(f" - Mapped {len(all_beacons)} beacons")
            print(f" - Only {len([d for d in data if d])} un-mapped scanners remaining!\n")
            remapped[i] = {"scanner": s1, "beacons": b1}
            data[i] = None
            found = True
            _save(data, remapped, compared)
            break
          _save(data, remapped, compared)
      if found:
        break
  _save(data, remapped, compared)
  print(f"PART 1: There are {len(all_beacons)} beacons in total")


def _save(data, remapped, compared):
  with _CACHE.open("w") as f:
    json.dump({"data": data, "remapped": remapped, "compared": sorted(compared)}, f, indent=2)


def _find_overlap(b0, b1):
  b1_rots = [[_rotate(b, rot) for b in b1] for rot in _rotation_matrices()]
  with Parallel(n_jobs=12, verbose=0) as parallel:
    results = parallel(delayed(_find_overlapping)(b0, b1) for b1 in b1_rots)
    results = [r for r in results if r]
    if results:
      return results[0]


def _find_overlapping(b0, b1):
  """First set of beacons (b0) are assumed to be in the correct coordinate system."""
  product = itertools.product(_combinations(b0), _combinations(b1))
  for (origin_wrt_sub, _, subset0), (s1_wrt_sub, b1_wrt_sub, subset1) in product:
    if subset0 == subset1:
      s1 = _recenter(s1_wrt_sub, center=origin_wrt_sub)
      b1 = [_recenter(b, center=origin_wrt_sub) for b in b1_wrt_sub]
      return s1, b1


def _combinations(beacons, k=12):
  """Iterate over all valid combinations of k points."""
  beacons = sorted(beacons)
  all_x, all_y, all_z = list(zip(*beacons))
  used = set()
  for center in itertools.product(all_x, all_y, all_z):
    for octant in itertools.product([-1, 1], [-1, 1], [-1, 1]):
      subset = tuple(b for b in beacons if _in_octant(b, octant, center=center))
      if len(subset) == k and subset not in used:
        used.add(subset)
        scanner_recentered = _recenter((0, 0, 0), subset[0])
        beacons_recentered = [_recenter(b, subset[0]) for b in beacons]
        subset_recentered = tuple(_recenter(p, subset[0]) for p in subset)
        yield scanner_recentered, beacons_recentered, subset_recentered


def _rotate(point, matrix):
  """Rotate point using a rotation matrix."""
  arr = np.array([[v] for v in point])
  x, y, z = (matrix @ arr).flatten()
  return int(x), int(y), int(z)


def _recenter(point, center):
  """Recenter a point."""
  x, y, z = point
  xo, yo, zo = center
  return x - xo, y - yo, z - zo


def _in_octant(point, octant, center=(0, 0, 0)):
  """Is this point in a given octant relative to some center."""
  for (c, co, sign) in zip(point, center, octant):
    if ((c - co) < 0 and sign == 1) or ((c - co) > 0 and sign == -1):
      return False
  return True


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


def _load():
  if _CACHE.exists():
    with _CACHE.open() as f:
      blob = json.load(f)
      data = [[tuple(x) for x in row] if row else None for row in blob["data"]]
      remapped = [{
        "scanner": tuple(row["scanner"]),
        "beacons": [tuple(x) for x in row["beacons"]]
      } if row else None for row in blob["remapped"]]
      compared = set(tuple(x) for x in blob["compared"])
      return data, remapped, compared

  data = _load_scanner_data()
  remapped = [None for _ in range(len(data))]
  remapped[0] = {"scanner": (0, 0, 0), "beacons": data[0]}
  data[0] = None
  return data, remapped, set()


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
  return scanner_data


def test_1():
  data = _load_scanner_data(test=True)
  s0 = data[0]
  exp = [
    (-618,-824,-621),
    (-537,-823,-458),
    (-447,-329,318),
    (404,-588,-901),
    (544,-627,-890),
    (528,-643,409),
    (-661,-816,-575),
    (390,-675,-793),
    (423,-701,434),
    (-345,-311,381),
    (459,-707,401),
    (-485,-357,347),
  ]
  exp = [_recenter(p, exp[0]) for p in exp]
  found = False
  for combo in _combinations(s0):
    if set(combo) == set(exp):
      found = True
  assert found


def test_2():
  data = _load_scanner_data(test=True)
  s0, s1 = data[:2]
  found = 0
  for i, rot in enumerate(_rotation_matrices(), start=1):
    print(f"Trying rotation {i}")
    s1_ = [_rotate(p, rot) for p in s1]
    for (scanner0, comb0), (scanner1, comb1) in itertools.product(_combinations(s0), _combinations(s1_)):
      if comb0 == comb1:
        found += 1
        assert _recenter(scanner1, scanner0) == (68, -1246, -43)
  assert found == 1


if __name__ == "__main__":
  #test_1()
  #test_2()
  part_1()
