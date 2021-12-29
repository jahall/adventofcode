from pathlib import Path


def part_1():
  """Simple part 1"""
  count = 0
  for _, out in _iter_rows():
    for o in out:
      if len(o) in {2, 3, 4, 7}:
        count += 1
  print(f"PART 1: Count of 1, 4, 7 and 8s is {count}")


# 1 =   c  f  (2)
# 7 = a c  f  (3)
# 4 =  bcd f  (4)

# 2 = a cde g (5)
# 3 = a cd fg (5)
# 5 = ab d fg (5)

# 0 = abc efg (6)
# 6 = ab defg (6)
# 9 = abcd fg (6)

# 8 = abcdefg (7)


def part_2():
  """Complex part 2"""
  count = 0
  for inp, out in _iter_rows():
    # 1. Find 1, 4, 7 and 8
    inp = sorted(inp, key=lambda x: len(x))
    encoder = {1: inp[0], 4: inp[2], 7: inp[1], 8: inp[9]}
    # 2. Pry apart 2, 3 and 5
    bd = encoder[4] - encoder[1]
    cf = encoder[1]
    encoder.update({
      2: [x for x in inp[3:6] if len(x - cf) == 4 and len(x - bd) == 4][0],
      3: [x for x in inp[3:6] if len(x - cf) == 3][0],
      5: [x for x in inp[3:6] if len(x - bd) == 3][0],
    })
    # 3. Pry apart 0, 6 and 9
    c = list(encoder[1] & encoder[2])[0]
    e = list(encoder[2] - encoder[3])[0]
    encoder.update({
      0: [x for x in inp[6:9] if c in x and e in x][0],
      6: [x for x in inp[6:9] if c not in x and e in x][0],
      9: [x for x in inp[6:9] if c in x and e not in x][0],
    })
    # 4. Decode
    decoder = {code: str(digit) for digit, code in encoder.items()}
    num_str = ""
    for o in out:
      num_str += decoder[o]
    count += int(num_str)
  print(f"PART 2: Total count is {count}")


def _iter_rows():
  input_file = Path(__file__).parent.parent / "data" / "day8.txt"
  with input_file.open() as f:
    for line in f:
      inp, out = line.strip().split("|")
      inp = [frozenset(code) for code in inp.strip().split()]
      out = [frozenset(code) for code in out.strip().split()]
      yield inp, out


if __name__ == "__main__":
  part_1()
  part_2()
