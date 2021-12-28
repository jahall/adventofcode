from collections import defaultdict
from pathlib import Path

class NaiveMonad:
  def __init__(self):
    self._instructions = []
    input_file = Path(__file__).parent.parent / "data" / "day24.txt"
    with input_file.open() as f:
      for line in f:
        parts = line.strip().split()
        operation = parts[0]
        variable = parts[1]
        variable_2 = None
        if len(parts) > 2:
          try:
            variable_2 = int(parts[2])
          except ValueError:
            variable_2 = parts[2]
        self._instructions.append((operation, variable, variable_2))

  def find_largest_model_num(self):
    """Find largest valid model number."""
    model_num = 99_999_999_999_999
    while True:
      model_num -= 1
      model_num_str = str(model_num)
      if model_num % 1000 == 0:
        print(f"\r{model_num}", end="")
      if "0" in model_num_str:
        continue
      try:
        result = self.run(model_num_str)
        if result == 0:
          break
      except RuntimeError:
        pass
    return model_num

  def run(self, inp):
    """Run monad on input."""
    inp = str(inp)
    inp_idx = 0
    state = {"w": 0, "x": 0, "y": 0, "z": 0}
    for op, a, b in self._instructions:
      a_val = state[a]
      b_val = self._var(state, b)
      if op == "inp":
        state[a] = int(inp[inp_idx])
        inp_idx += 1
      elif op == "add":
        state[a] += b_val
      elif op == "mul":
        state[a] *= b_val
      elif op == "div":
        if b_val == 0:
          raise RuntimeError("Attempted to perform div with b=0")
        state[a] = a_val // b_val
      elif op == "mod":
        if a_val < 0:
          raise RuntimeError("Attempted to perform mod with a<0")
        elif b_val <= 0:
          raise RuntimeError("Attempted to perform mod with b<=0")
        state[a] = a_val % b_val
      elif op == "eql":
        state[a] = int(a_val == b_val)
    return state["z"]

  def _var(self, state, v):
    if v is None:
      return None
    elif isinstance(v, int):
      return v
    else:
      return state[v]


class Monad:
  def __init__(self):
    input_file = Path(__file__).parent.parent / "data" / "day24.txt"
    with input_file.open() as f:
      lines = [line.strip().split()[-1] for line in f]
    self._params = list(zip(lines[5::18], lines[4::18], lines[15::18]))
    self._params = [tuple(int(i) for i in params) for params in self._params]

  def find_valid_model_nums(self):
    """Find all valid model numbers."""
    valid_tails = self._valid_tails(level=0)
    return {int(num) for num in valid_tails[0]}

  def _valid_tails(self, level):
    # Mapping from z_in to set of valid output heads
    if level == 14:
      return {0: {""}}

    valid_tails = defaultdict(set)
    sub, div, offset = self._params[level]
    for z_out, tails in self._valid_tails(level + 1).items():
      # Special pass-through case
      for z_in in [z_out * div + i for i in range(div)]:
        for w in range(1, 10):
          if z_in % 26 + sub == w:
            valid_tails[z_in].update({f"{w}{tail}" for tail in tails})
      # Generic case
      for w in range(1, 10):
        z_pre = z_out - w - offset
        if z_pre < 0 or z_pre % 26 != 0:
          continue
        z_pre = z_pre // 26
        for z_in in [z_pre * div + i for i in range(div)]:
          if z_in % 26 + sub != w:
            valid_tails[z_in].update({f"{w}{tail}" for tail in tails})
    return valid_tails

  def run(self, inp):
    """Run monad on input."""
    z = 0
    for level, w in enumerate(str(inp)):
      z = self._op(z, w, level)
    return z

  def _op(self, z, w, level):
    sub, div, offset = self._params[level]
    w = int(w)
    x = z % 26 + sub == w
    z = z // div
    if not x:
      z = 26 * z + w + offset
    return z


def part_1():
  """Easy part 1."""
  monad = Monad()
  largest_num = max(monad.find_valid_model_nums())
  assert monad.run(largest_num) == 0
  print(f"PART 1: Largest num accepted by MONAD: {largest_num}")
  
 
def part_2():
  """Complex part 2."""
  monad = Monad()
  smallest_num = min(monad.find_valid_model_nums())
  assert monad.run(smallest_num) == 0
  print(f"PART 2: Smallest num accepted by MONAD: {smallest_num}")
  

if __name__ == "__main__":
  part_1()
  part_2()
