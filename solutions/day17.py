from dataclasses import dataclass
import itertools

def part_1():
  """Simple part 1"""
  target = _load_target()
  # 1. Ensure that dropping straight down on the target is possible
  _find_min_init_xv(target)
  # 2. Missile will pass through zero at the same speed it started at...
  #  but in the opposite direction, so just set it so it hits the bottom
  #  of the target area on the step after it passes back through zero
  init_y_vel = -target.ymin - 1
  # 3. Find highest y
  highest_y = sum(range(init_y_vel + 1))
  print(f"PART 1: Highest y is {highest_y} (init velocity of {init_y_vel})")


def part_2():
  """Complex part 2"""
  target = _load_target()
  min_xv, max_xv = _find_min_init_xv(target), target.xmax
  min_yv, max_yv = target.ymin, -target.ymin - 1
  successes = 0
  # just brute-force it :)
  for xv, yv in itertools.product(range(min_xv, max_xv + 1), range(min_yv, max_yv + 1)):
    if _hits_target(target, xv, yv):
      successes += 1
  print(f"PART 2: There are {successes} possible initial velocities")


def _find_min_init_xv(target):
  min_init_xv, terminal_x = 0, 0
  while True:
    min_init_xv += 1
    terminal_x += min_init_xv
    if target.xmin <= terminal_x <= target.xmax:
      break
    elif terminal_x > target.xmax:
      raise RuntimeError("Doh")
  return min_init_xv


def _hits_target(target, xv, yv):
  x, y = 0, 0
  while True:
    x += xv
    y += yv
    xv = max(xv - 1, 0)
    yv = yv - 1
    if target.xmin <= x <= target.xmax and target.ymin <= y <= target.ymax:
      return True
    elif x > target.xmax or y < target.ymin:
      return False


@dataclass(frozen=True)
class Target:
  xmin: int
  xmax: int
  ymin: int
  ymax: int


def _load_target():
  return Target(xmin=211, xmax=232, ymin=-124, ymax=-69)


if __name__ == "__main__":
  part_1()
  part_2()
