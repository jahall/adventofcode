from os import stat
from queue import PriorityQueue

class Burrow:
  """Representation of the burrow.
  
  State is a mapping from room num (0 = hallway) and
  location to amphimod presently occupying that space.
  """
  _allowable_hallway_locs = {0, 1, 3, 5, 7, 9, 10}

  def __init__(self, state, energy_used=0):
    self._state = state
    self._energy_used = energy_used

  @classmethod
  def from_puzzle(cls, puzzle):
    """Init from puzzle list of lists."""
    state = {
      (room, loc): amphipod
      for room, amphipods in enumerate(puzzle, start=1)
      for loc, amphipod in enumerate(amphipods)
    }
    return cls(state)

  @property
  def is_complete(self):
    """Have we arrived at the correct end-state?"""
    return self._state == {
      (1, 0): "A",
      (1, 1): "A",
      (2, 0): "B",
      (2, 1): "B",
      (3, 0): "C",
      (3, 1): "C",
      (4, 0): "D",
      (4, 1): "D",
    }

  @property
  def energy_used(self):
    """How much energy has been used to get into this state?"""
    return self._energy_used

  def new(self, old_loc, new_loc, steps):
    """Create a new burrow."""
    new_state = self._state.copy()
    amphipod = new_state.pop(old_loc)
    new_state[new_loc] = amphipod
    extra_energy = self._calc_energy_used(amphipod, steps)
    return Burrow(new_state, energy_used=self._energy_used + extra_energy)

  def iter_possible_next_moves(self):
    """Iter over all possible next moves."""
    for (room, loc), amphipod in self._state.items():
      if room == 0:  # hallway
        yield from self._iter_possible_hallway_moves(self, amphipod, loc, can_stay_in_hall=False)
      elif not self._arrived(amphipod, room, loc):
        hall_loc = room * 2
        if loc == 1:
          burrow = self.new((room, loc), (0, hall_loc), steps=1)
        elif not self.occupied(room, 1):
          burrow = self.new((room, loc), (0, hall_loc), steps=2)
        else:
          continue
        yield from self._iter_possible_hallway_moves(burrow, amphipod, hall_loc, can_stay_in_hall=True)

  @classmethod
  def _iter_possible_hallway_moves(cls, burrow, amphipod, loc, can_stay_in_hall):
    for rng in [range(loc - 1, -1, -1), range(loc + 1, 11)]:
      steps = 0
      for new_loc in rng:
        steps += 1
        if burrow.occupied(0, new_loc):
          break  # someone is here so can't move passed them
        elif can_stay_in_hall and new_loc in cls._allowable_hallway_locs:
          yield burrow.new((0, loc), (0, new_loc), steps)
        elif burrow.outside_correct_room_and_room_ready(amphipod, new_loc):
          side_room = new_loc // 2
          if not burrow.occupied(side_room, 0):
            yield burrow.new((0, loc), (side_room, 0), steps + 2)
          else:
            yield burrow.new((0, loc), (side_room, 1), steps + 1)

  def outside_correct_room_and_room_ready(self, amphipod, hall_loc):
    """Is this amphipod outside the correct room...and can it go in?"""
    correct_rooms = {"A": 1, "B": 2, "C": 3, "D": 4}
    correct_room = correct_rooms[amphipod]
    if correct_room * 2 != hall_loc:
      return False
    occupant_1 = self._state.get((correct_room, 0))
    occupant_2 = self._state.get((correct_room, 1))
    return (
      (occupant_1 is None or occupant_1 == amphipod) and
      (occupant_2 is None or occupant_2 == amphipod)
    )

  def occupied(self, room, loc):
    """Is this location already occupied?"""
    return (room, loc) in self._state

  def _arrived(self, amphipod, room, loc):
    correct_rooms = {"A": 1, "B": 2, "C": 3, "D": 4}
    correct_room = correct_rooms[amphipod]
    if room != correct_room:
      return False
    if loc == 1 and self._state[(room, 0)] != amphipod:
      return False
    return True

  @staticmethod
  def _calc_energy_used(amphimod, steps):
    energy_map = {"A": 1, "B": 10, "C": 100, "D": 1000}
    return energy_map[amphimod] * steps

  def __eq__(self, other):
    return self._energy_used == other._energy_used

  def __lt__(self, other):
    return self._energy_used < other._energy_used

  def __repr__(self):
    parts = []
    for room in [1, 2, 3, 4]:
      part = f"{self._state.get((room, 0), '-')}{self._state.get((room, 1), '-')}"
      parts.append(part)
    return ",".join(parts)


def part_1():
  """Easy part 1."""
  burrow = Burrow.from_puzzle(_load_start_state())
  queue = PriorityQueue()
  queue.put(burrow)
  finished = False
  i = 0
  while not finished:
    burrow = queue.get()  # pops burrow with least energy used so far
    i += 1
    if i % 1000 == 0:
      print(f"\rEnergy used so far: {burrow.energy_used} ({burrow})", end="")
    for next_burrow in burrow.iter_possible_next_moves():
      if next_burrow.is_complete:
        burrow = next_burrow
        finished = True
        break
      queue.put(next_burrow)
  print(f"\nFinal burrow: {burrow}")
  print(f"PART 1: Least energy required to sort the amphipods is {burrow.energy_used}")
  
 
def part_2():
  """Complex part 2."""
  pass


def _load_start_state(test=False):
  if test:
    return [
      ["A", "B"],
      ["D", "C"],
      ["C", "B"],
      ["A", "D"],
    ]
  return [
    ["B", "D"],
    ["A", "A"],
    ["D", "B"],
    ["C", "C"],
  ]
  

if __name__ == "__main__":
  part_1()
  part_2()
