from queue import PriorityQueue

class Burrow:
  """Representation of the burrow.
  
  State is a mapping from room num (0 = hallway) and
  location to amphimod presently occupying that space.
  """
  _allowable_hallway_locs = {0, 1, 3, 5, 7, 9, 10}

  def __init__(self, state, energy_used=0, room_size=2):
    self._state = state
    self._energy_used = energy_used
    self._room_size = room_size
    self._completed_state = {
      (i, j): a
      for i, a in enumerate("ABCD", start=1)
      for j in range(self._room_size)
    }

  @classmethod
  def from_puzzle(cls, puzzle):
    """Init from puzzle list of lists."""
    state = {
      (room, loc): amphipod
      for room, amphipods in enumerate(puzzle, start=1)
      for loc, amphipod in enumerate(amphipods)
    }
    return cls(state, room_size=len(puzzle[0]))

  @property
  def is_complete(self):
    """Have we arrived at the correct end-state?"""
    return self._state == self._completed_state

  @property
  def energy_used(self):
    """How much energy has been used to get into this state?"""
    return self._energy_used

  @property
  def state_hash(self):
    """Return a hash of the state."""
    return hash(frozenset(self._state.items()))

  def new(self, old_loc, new_loc, steps):
    """Create a new burrow."""
    new_state = self._state.copy()
    amphipod = new_state.pop(old_loc)
    new_state[new_loc] = amphipod
    extra_energy = self._calc_energy_used(amphipod, steps)
    return Burrow(
      new_state,
      energy_used=self._energy_used + extra_energy,
      room_size=self._room_size,
    )

  def iter_possible_next_moves(self):
    """Iter over all possible next moves."""
    for (room, loc), amphipod in self._state.items():
      if room == 0:  # hallway
        yield from self._iter_possible_hallway_moves(self, amphipod, loc, can_stay_in_hall=False)
      elif not self._arrived(amphipod, room, loc):
        hall_loc = room * 2
        blocked = any(self.occupied(room, i) for i in range(loc + 1, self._room_size))
        if blocked:
          continue
        steps_to_hall = self._room_size - loc
        burrow = self.new((room, loc), (0, hall_loc), steps=steps_to_hall)
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
          for i in range(burrow._room_size):
            if not burrow.occupied(side_room, i):
              yield burrow.new((0, loc), (side_room, i), steps=steps + burrow._room_size - i)

  def outside_correct_room_and_room_ready(self, amphipod, hall_loc):
    """Is this amphipod outside the correct room...and can it go in?"""
    correct_rooms = {"A": 1, "B": 2, "C": 3, "D": 4}
    correct_room = correct_rooms[amphipod]
    if correct_room * 2 != hall_loc:
      return False
    occupants = {
      self._state[(correct_room, i)]
      for i in range(self._room_size) if (correct_room, i) in self._state
    }
    return not occupants or occupants == {amphipod}

  def occupied(self, room, loc):
    """Is this location already occupied?"""
    return (room, loc) in self._state

  def _arrived(self, amphipod, room, loc):
    correct_rooms = {"A": 1, "B": 2, "C": 3, "D": 4}
    correct_room = correct_rooms[amphipod]
    if room != correct_room:
      return False
    amphipods_below = {
      self._state[(correct_room, i)]
      for i in range(loc) if (correct_room, i) in self._state
    }
    arrived = not amphipods_below or amphipods_below == {amphipod}
    return arrived

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
      part = "".join(self._state.get((room, i), "-") for i in range(self._room_size))
      parts.append(part)
    return ",".join(parts)


def part_1():
  """Easy part 1."""
  burrow = Burrow.from_puzzle(_load_start_state(test=False))
  burrow = _solve(burrow)
  print(f"\nFinal burrow: {burrow}")
  print(f"PART 1: Least energy required to sort the amphipods is {burrow.energy_used}")
  
 
def part_2():
  """Complex part 2."""
  burrow = Burrow.from_puzzle(_load_start_state(test=False, part=2))
  burrow = _solve(burrow)
  print(f"\nFinal burrow: {burrow}")
  print(f"PART 2: Least energy required to sort the amphipods is {burrow.energy_used}")


def _solve(burrow):
  queue = PriorityQueue()
  queue.put(burrow)
  used = set()
  i = 0
  while True:
    i += 1
    burrow = queue.get()  # pops burrow with least energy used so far
    if burrow.is_complete:
      break
    elif burrow.state_hash in used:
      continue
    used.add(burrow.state_hash)
    if i % 1000 == 0:
      print(f"\rEnergy used so far: {burrow.energy_used} ({burrow})", end="")
    for next_burrow in burrow.iter_possible_next_moves():
      if next_burrow.state_hash not in used:
        queue.put(next_burrow)
  return burrow


def _load_start_state(test=False, part=1):
  if test:
    state = [
      ["A", "B"],
      ["D", "C"],
      ["C", "B"],
      ["A", "D"],
    ]
  else:
    state = [
      ["B", "D"],
      ["A", "A"],
      ["D", "B"],
      ["C", "C"],
    ]
  if part == 2:
    extension = [
      ["D", "D"],
      ["B", "C"],
      ["A", "B"],
      ["C", "A"],
    ]
    state = [
      a[:1] + b + a[-1:]
      for a, b in zip(state, extension)
    ]
  return state
  

if __name__ == "__main__":
  part_1()
  part_2()
