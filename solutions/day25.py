from pathlib import Path

import numpy as np


def part_1():
  """Easy part 1."""
  ef_herd, sf_herd = _load_herds()
  steps = 0
  while True:
    steps += 1
    print(f"\rStep {steps}", end="")
    ef_moves = ef_herd * _shift(1 - ef_herd - sf_herd, direction="left")
    ef_herd = ef_herd - ef_moves + _shift(ef_moves, direction="right")
    sf_moves = sf_herd * _shift(1 - ef_herd - sf_herd, direction="up")
    sf_herd = sf_herd - sf_moves + _shift(sf_moves, direction="down")
    if ef_moves.sum() + sf_moves.sum() == 0:
      break
  print(f"\nPART 1: Converged in {steps} steps")
  
 
def part_2():
  """Complex part 2."""
  print("PART 2: Done!")


def _shift(x, direction):
  if direction == "left":
    return np.hstack([x[:,1:], x[:,:1]])
  elif direction == "up":
    return np.vstack([x[1:,:], x[:1,:]])
  elif direction == "right":
    return np.hstack([x[:,-1:], x[:,:-1]])
  elif direction == "down":
    return np.vstack([x[-1:,:], x[:-1,:]])


def _load_herds():
  input_file = Path(__file__).parent.parent / "data" / "day25.txt"
  ef_herd, sf_herd = [], []
  with input_file.open() as f:
    for line in f:
      ef_herd.append([1 if cell == ">" else 0 for cell in line.strip()])
      sf_herd.append([1 if cell == "v" else 0 for cell in line.strip()])
  ef_herd = np.array(ef_herd).astype(int)
  sf_herd = np.array(sf_herd).astype(int)
  return ef_herd, sf_herd
  

if __name__ == "__main__":
  part_1()
  part_2()
