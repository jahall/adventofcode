import copy
import itertools
import json
import math
from pathlib import Path

class Pair:
  def __init__(self, left, right=None):
    self.left = left
    self.right = right
    if right is not None:
      if isinstance(left, int):
        self.left = Pair(left)
      if isinstance(right, int):
        self.right = Pair(right)

  @property
  def is_number(self):
    return self.right is None

  @property
  def magnitude(self):
    if self.is_number:
      return self.left
    return 3 * self.left.magnitude + 2 * self.right.magnitude

  @classmethod
  def from_list(cls, pair_list):
    """Init from a list."""
    if isinstance(pair_list, int):
      return Pair(left=pair_list)
    left, right = pair_list
    return Pair(
      left=Pair.from_list(left),
      right=Pair.from_list(right),
    )

  def copy(self):
    """Important for part 2!!"""
    return copy.deepcopy(self)

  def __add__(self, other):
    pair = Pair(self, other)
    pair.reduce()
    return pair

  def __eq__(self, other):
    return str(self) == str(other)

  def add(self, value, side):
    if self.is_number:
      self.left += value
    else:
      sub_pair = getattr(self, side)
      sub_pair.add(value, side)

  def reduce(self, verbose=False):
    """Perform reduction."""
    if verbose:
      print(self)
    while True:
      altered_explode = self._reduce("explode")
      if verbose and altered_explode:
        print(self)
      altered_split, _, _ = self._reduce_once("split")
      if verbose and altered_split:
        print(self)
      if not altered_explode and not altered_split:
        return self

  def _reduce(self, type):
    any_altered = False
    while True:
      altered, _, _ = self._reduce_once(type)
      any_altered = any_altered or altered
      if not altered:
        return any_altered
    
  def _reduce_once(self, type, level=1):
    """Perform one reduction step."""
    # 1. Handle splits
    if self.is_number:
      if type == "split" and self.magnitude >= 10:
        self.split()
        return True, None, None
      return False, None, None
    # 2. Handle explosion
    if level > 4:
      if type == "explode":
        l, r = self.explode()
        return True, l, r
      return False, None, None
    # 3. Handle left recursion
    altered, lval, rval = self.left._reduce_once(type, level=level + 1)
    if altered:
      if rval is not None:
        self.right.add(rval, side="left")
        rval = None
      return True, lval, rval
    # 4. Handle right recursion
    altered, lval, rval = self.right._reduce_once(type, level=level + 1)
    if altered:
      if lval is not None:
        self.left.add(lval, side="right")
        lval = None
      return True, lval, rval
    return False, None, None

  def explode(self):
    """Perform explosion."""
    if not self.is_number:
      l = self.left.magnitude
      r = self.right.magnitude
      self.left = 0
      self.right = None
      return l, r

  def split(self):
    """Perform split."""
    if self.is_number:
      value = self.magnitude
      self.left = Pair(math.floor(value / 2))
      self.right = Pair(math.ceil(value / 2))

  def __repr__(self):
    return self._repr()

  def _repr(self, level=1):
    if self.is_number:
      if self.magnitude >= 10:
        return f"\u001b[31m{self.left}\u001b[0m"
      return f"{self.left}"
    left_str = self.left._repr(level + 1)
    right_str = self.right._repr(level + 1)
    if level > 4:
      return f"\u001b[32m({left_str}\u001b[0m,\u001b[32m{right_str}\u001b[32m)\u001b[0m"
    return f"[{left_str},{right_str}]"


def tests():
  """Some tests."""
  # 1. Single explodes
  p = Pair.from_list([[[[[9,8],1],2],3],4])
  p._reduce_once("explode")
  assert p == Pair.from_list([[[[0,9],2],3],4])

  p = Pair.from_list([7,[6,[5,[4,[3,2]]]]])
  p._reduce_once("explode")
  assert p == Pair.from_list([7,[6,[5,[7,0]]]])

  p = Pair.from_list([[6,[5,[4,[3,2]]]],1])
  p._reduce_once("explode")
  assert p == Pair.from_list([[6,[5,[7,0]]],3])

  p = Pair.from_list([[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]])
  p._reduce_once("explode")
  assert p == Pair.from_list([[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]])

  p = Pair.from_list([[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]])
  p._reduce_once("explode")
  assert p == Pair.from_list([[3,[2,[8,0]]],[9,[5,[7,0]]]])

  # 2. Full sums
  p = Pair(1, 1) + Pair(2, 2) + Pair(3, 3) + Pair(4, 4)
  assert p == Pair.from_list([[[[1,1],[2,2]],[3,3]],[4,4]])

  p = Pair(1, 1) + Pair(2, 2) + Pair(3, 3) + Pair(4, 4) + Pair(5, 5) + Pair(6, 6)
  assert p == Pair.from_list([[[[5,0],[7,4]],[5,5]],[6,6]])

  p1 = Pair.from_list([[[[4,3],4],4],[7,[[8,4],9]]])
  p2 = Pair.from_list([1,1])
  ex = Pair.from_list([[[[0,7],4],[[7,8],[6,0]]],[8,1]])
  assert p1 + p2 == ex

  pairs_list = [
    [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]],
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]],
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]],
    [7,[5,[[3,8],[1,4]]]],
    [[2,[2,2]],[8,[8,1]]],
    [2,9],
    [1,[[[9,3],9],[[9,0],[0,7]]]],
    [[[5,[7,4]],7],1],
    [[[[4,2],2],6],[8,7]],
  ]
  pairs = [Pair.from_list(lst) for lst in pairs_list]
  expected = Pair.from_list([[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]])
  result = pairs[0]
  for p in pairs[1:]:
    result += p
  assert result == expected

  pairs_list = [
    [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]],
    [[[5,[2,8]],4],[5,[[9,9],0]]],
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]],
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]],
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]],
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]],
    [[[[5,4],[7,7]],8],[[8,3],8]],
    [[9,3],[[9,9],[6,[4,9]]]],
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]],
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]],
  ]
  pairs = [Pair.from_list(lst) for lst in pairs_list]
  expected = Pair.from_list([[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]])
  result = pairs[0]
  for p in pairs[1:]:
    result += p
  assert result == expected

  # 3. Magnitudes
  p = Pair.from_list([[[[0,7],4],[[7,8],[6,0]]],[8,1]])
  assert p.magnitude == 1384

  p = Pair.from_list([[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]])
  assert p.magnitude == 3488

  # 4. Largest magnitude
  pairs = [Pair.from_list(lst) for lst in pairs_list]
  magnitude = max(
    (p1.copy() + p2.copy()).magnitude
    for p1, p2 in itertools.permutations(pairs, 2)
  )
  assert magnitude == 3993


def part_1():
  """Simple part 1"""
  pairs = [Pair.from_list(lst) for lst in _load_numbers()]
  result = pairs[0]
  for p in pairs[1:]:
    result += p
  print(f"PART 1: Magnitude of the final sum is: {result.magnitude}")


def part_2():
  """Complex part 2"""
  pairs = [Pair.from_list(lst) for lst in _load_numbers()]
  magnitude = max(
    (p1.copy() + p2.copy()).magnitude
    for p1, p2 in itertools.permutations(pairs, 2)
  )
  print(f"PART 2: Largest magnitude is: {magnitude}")


def _load_numbers():
  input_file = Path(__file__).parent.parent / "data" / "day18.txt"
  numbers = []
  with input_file.open() as f:
    for line in f:
      numbers.append(json.loads(line.strip()))
  return numbers


if __name__ == "__main__":
  tests()
  part_1()
  part_2()
