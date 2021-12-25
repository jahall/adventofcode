from pathlib import Path

class Monad:
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

  def run(self, inp):
    """Run monad on input."""
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
    return state["z"]

  def _var(self, state, v):
    if v is None:
      return None
    elif isinstance(v, int):
      return v
    else:
      return state[v]


def part_1():
  """Easy part 1."""
  monad = Monad()
  model_num = 99_999_999_999_999
  while True:
    model_num -= 1
    model_num_str = str(model_num)
    if model_num % 1000 == 0:
      print(f"\r{model_num}", end="")
    if "0" in model_num_str:
      continue
    try:
      result = monad.run(model_num_str)
      if result == 0:
        break
    except RuntimeError:
      pass
  print(result)
  
 
def part_2():
  """Complex part 2."""
  pass
  

if __name__ == "__main__":
  part_1()
  part_2()
