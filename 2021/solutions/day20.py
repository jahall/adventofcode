from pathlib import Path


def part_1():
  """Simple part 1"""
  image = _find_pixels(n=2)
  print(f"PART 1: {len(image)} pixels are lit after 2 iterations.")


def part_2():
  """Complex part 2"""
  image = _find_pixels(n=50)
  print(f"PART 1: {len(image)} pixels are lit after 50 iterations.")


def _find_pixels(n, test=False):
  algo, image = _load_image_data(test=test)
  image = {
    (r, c)
    for r, row in enumerate(image)
    for c, pix in enumerate(row)
    if pix
  }
  if test:
    _show(0, image)
  default = 0
  for i in range(1, n + 1):
    new_image = set()
    rmin, rmax, cmin, cmax = _image_bounds(image)
    for r in range(rmin - 1, rmax + 2):
      for c in range(cmin - 1, cmax + 2):
        index = _output(image, r, c, rmin, rmax, cmin, cmax, default=default)
        if algo[index]:
          new_image.add((r, c))
    image = new_image
    if default == 0 and algo[0] == 1:
      default = 1
    elif default == 1 and algo[-1] == 0:
      default = 0
    if test:
      _show(i, image, default=default)
  return image

  
def _output(image, r0, c0, rmin, rmax, cmin, cmax, default=0):
  arr = []
  for r in [r0 - 1, r0, r0 + 1]:
    for c in [c0 - 1, c0, c0 + 1]:
      if rmin <= r <= rmax and cmin <= c <= cmax:
        pix = 1 if (r, c) in image else 0
      else:
        pix = default
      arr.append(pix)
  return _arr_to_index(arr)


def _arr_to_index(arr):
  return sum(pix * 2 ** i for i, pix in enumerate(reversed(arr)))


def _image_bounds(image):
  rmin = min(r for r, _ in image)
  rmax = max(r for r, _ in image)
  cmin = min(c for _, c in image)
  cmax = max(c for _, c in image)
  return rmin, rmax, cmin, cmax


def _show(i, image, default=0):

  def _to_pix(val):
    return "#" if val else "."

  print(f"\nImage {i} (surrounded by {_to_pix(default)})\n")
  rmin, rmax, cmin, cmax = _image_bounds(image)
  for r in range(rmin, rmax + 1):
    for c in range(cmin, cmax + 1):
      print(_to_pix((r, c) in image), end="")
    print("\n", end="")


def _load_image_data(test=False):
  suffix = "_test" if test else ""
  input_file = Path(__file__).parent.parent / "data" / f"day20{suffix}.txt"
  with input_file.open() as f:
    algorithm = _row_to_pixels(next(f).strip())
    next(f)
    image = [_row_to_pixels(row.strip()) for row in f]
  return algorithm, image


def _row_to_pixels(row: str):
  return [0 if pix == "." else 1 for pix in row]


if __name__ == "__main__":
  part_1()
  part_2()
