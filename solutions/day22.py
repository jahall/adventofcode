import itertools
from pathlib import Path


class NaiveReactorCore:
  def __init__(self):
    self._on = set()

  @property
  def num_cubes_on(self):
    """How many points are on?"""
    return len(self._on)

  def switch(self, cube, on=True, verbose=False):
    """Turn all points in a cube on or off."""
    xmin, xmax, ymin, ymax, zmin, zmax = cube
    points = {
      (x, y, z)
      for x in range(xmin, xmax + 1)
      for y in range(ymin, ymax + 1)
      for z in range(zmin, zmax + 1)
    }
    if on:
      self._on = self._on | points
    else:
      self._on = self._on - points


class Block:
  """Helpful wrapper around a block.
  
  Note that definition is NOT inclusive of the second point.
  """
  def __init__(self, x1, x2, y1, y2, z1, z2, inclusive=False):
    self._x1 = x1
    self._x2 = x2 + (1 if inclusive else 0)
    self._y1 = y1
    self._y2 = y2 + (1 if inclusive else 0)
    self._z1 = z1
    self._z2 = z2 + (1 if inclusive else 0)

  @property
  def volume(self) -> int:
    """The volume of this block."""
    return (
      (self._x2 - self._x1) *
      (self._y2 - self._y1) *
      (self._z2 - self._z1)
    )

  @property
  def empty(self):
    """Is this block empty?"""
    return self.volume == 0

  def inside(self, other) -> bool:
    """Is this block inside another block?"""
    return (
      self._x1 >= other._x1 and
      self._x2 <= other._x2 and
      self._y1 >= other._y1 and
      self._y2 <= other._y2 and
      self._z1 >= other._z1 and
      self._z2 <= other._z2
    )

  def outside(self, other) -> bool:
    """Is this block outside another block?"""
    return (
      self._x1 >= other._x2 or
      self._x2 <= other._x1 or
      self._y1 >= other._y2 or
      self._y2 <= other._y1 or
      self._z1 >= other._z2 or
      self._z2 <= other._z1
    )

  def overlaps(self, other) -> bool:
    """Does this block overlap with another?"""
    return not self.outside(other)

  def contiguous(self, other) -> bool:
    """Can these blocks be collapsed together?
    
    Do they same two of the same axes and touch on the third?
    """
    xs = {self._x1, self._x2, other._x1, other._x2}
    ys = {self._y1, self._y2, other._y1, other._y2}
    zs = {self._z1, self._z2, other._z1, other._z2}
    return len(xs) + len(ys) + len(zs) == 7

  def union(self, *others) -> "Block":
    """Find the smallest block that contains self and all others."""
    xs, ys, zs = self._find_coordinate_list(others)
    return Block(min(xs), max(xs), min(ys), max(ys), min(zs), max(zs))

  def iter_partitions(self, *others):
    """Iterate over the sub-blocks defined by the overlap.
    
    Note: when others is empty, this would just yield this block.
    When there is just one other, this will yield between 1 and 18 partitions.
    """
    xs, ys, zs = self._find_coordinate_list(others)
    xs.sort()
    ys.sort()
    zs.sort()
    for x1, x2 in zip(xs[:-1], xs[1:]):
      for y1, y2 in zip(ys[:-1], ys[1:]):
        for z1, z2 in zip(zs[:-1], zs[1:]):
          partition = Block(x1, x2, y1, y2, z1, z2)
          if not partition.empty:
            yield partition

  def _find_coordinate_list(self, others):
    xs = [self._x1, self._x2]
    ys = [self._y1, self._y2]
    zs = [self._z1, self._z2]
    for other in others:
      xs.extend([other._x1, other._x2])
      ys.extend([other._y1, other._y2])
      zs.extend([other._z1, other._z2])
    return xs, ys, zs

  def __hash__(self):
    return hash((self._x1, self._x2, self._y1, self._y2, self._z1, self._z2))

  def __repr__(self):
    return (
      f"x={self._x1}..{self._x2 - 1},"
      f"y={self._y1}..{self._y2 - 1},"
      f"z={self._z1}..{self._z2 - 1}"
    )


class ReactorCore:
  def __init__(self):
    self._on_blocks = set()

  @property
  def num_cubes_on(self):
    """How many cubes are on?"""
    # On blocks are non-overlapping by design
    return sum(on.volume for on in self._on_blocks)

  def switch(self, switch_block, on=True, verbose=False):
    """Turn this block on or off."""
    if isinstance(switch_block, tuple):
      kw = {"inclusive": True}
      switch_block = Block(*switch_block, **kw)
    if verbose: print(f"TURNING {'ON' if on else 'OFF'} {switch_block}")
    # If turning on an existing on-patch just ignore
    if on and any(switch_block.inside(b) for b in self._on_blocks):
      if verbose: print(f"- Already inside an existing on block")
      return
    # Or if turning off an existing off-patch just ignore
    if not on and all(switch_block.outside(b) for b in self._on_blocks):
      if verbose: print(f"- Already outside all existing on blocks")
      return
    self._discard_on_blocks_inside_switch_block(switch_block, verbose)
    # Find all current on blocks which overlap with the switch block, remove
    # them from current on blocks and iterate over resulting partitions
    overlapping = [b for b in self._on_blocks if b.overlaps(switch_block)]
    for b in overlapping:
      self._on_blocks.discard(b)
    if on:
      self._on_blocks.add(switch_block)
    if overlapping:
      self._repartition_overlapping_blocks(switch_block, overlapping, verbose)

  def _discard_on_blocks_inside_switch_block(self, switch_block, verbose):
    # These will get consumed regardless so don't need them.
    n = len(self._on_blocks)
    self._on_blocks = {b for b in self._on_blocks if not b.inside(switch_block)}
    discarded = n - len(self._on_blocks)
    if verbose and discarded:
      print(f"- ({len(self._on_blocks)}) Discarded {discarded} blocks inside the switch")

  def _repartition_overlapping_blocks(self, switch_block, overlapping, verbose):
    n_partitions = 0
    for partition in switch_block.iter_partitions(*overlapping):
      if partition.outside(switch_block) and any(partition.inside(b) for b in overlapping):
        n_partitions += 1
        self._on_blocks.add(partition)
    if verbose:
      print(f"- ({len(self._on_blocks)}) Repartitioned {len(overlapping)} overlapping blocks to {n_partitions} new blocks")
    union = switch_block.union(*overlapping)
    self._consolidate(union, verbose)

  def _consolidate(self, block, verbose):
    """Consolidate blocks within a given block to prevent explosion!"""
    n = len(self._on_blocks)
    eligible = {b for b in self._on_blocks if b.inside(block)}
    ne = len(eligible)
    while True:
      combined = set()
      # attempt to collapse big blocks first (seems to be useful in practise...)
      sorted_eligible = sorted(eligible, key=lambda x: x.volume, reverse=True)
      for b1, b2 in itertools.product(sorted_eligible, sorted_eligible):
        if b1 in combined or b2 in combined or not b1.contiguous(b2):
          continue
        b = b1.union(b2)
        self._on_blocks.discard(b1)
        self._on_blocks.discard(b2)
        self._on_blocks.add(b)
        eligible.discard(b1)
        eligible.discard(b2)
        eligible.add(b)
        combined.update({b1, b2})
      if not combined:
        nc = len(eligible)
        if verbose and nc < ne:
          print(f"- ({len(self._on_blocks)}) Consolidated {ne} partitioned blocks down to {nc}")
        break


def part_1():
  """Easy part 1."""
  core = NaiveReactorCore()
  for i, (on, cube) in enumerate(_iter_reboot_steps(n=20), start=1):
    print(f"Iteration {i}")
    core.switch(cube, on=on, verbose=True)
  print(f"PART 1: {core.num_cubes_on} cubes are on in the -50..50 grid")
  

def part_2():
  """Complex part 2."""
  core = ReactorCore()
  steps = list(_iter_reboot_steps())
  n_steps = len(steps)
  for i, (on, cube) in enumerate(steps, start=1):
    print(f"\nIteration {i} / {n_steps}")
    core.switch(cube, on=on, verbose=True)
  print(f"PART 2: {core.num_cubes_on} cubes are on in the full grid")


def _iter_reboot_steps(n=None):
  input_file = Path(__file__).parent.parent / "data" / "day22.txt"
  with input_file.open() as f:
    i = 1
    for line in f:
      on_off, cube = line.strip().split()
      on = on_off == "on"
      x, y, z = cube.split(",")
      xmin, _, xmax = x[2:].split(".")
      ymin, _, ymax = y[2:].split(".")
      zmin, _, zmax = z[2:].split(".")
      cube = int(xmin), int(xmax), int(ymin), int(ymax), int(zmin), int(zmax)
      yield on, cube
      if n and i == n:
        break
      i += 1

if __name__ == "__main__":
  part_1()
  part_2()
