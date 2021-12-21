from collections import defaultdict
from pathlib import Path

class DeterministicDice:
  def __init__(self):
    self._state = 1

  def roll(self):
    value = self._state
    self._state += 1
    if self._state == 101:
      self._state = 1
    return value


def part_1():
  """Easy part 1."""
  positions = _load_starting_positions()
  n_players = len(positions)
  scores = [0] * len(positions)
  rolls = 0
  dice = DeterministicDice()
  while True:
    finished = False
    for player in range(n_players):
      roll = sum(dice.roll() for _ in range(3))
      rolls += 3
      next_position = (positions[player] + roll - 1) % 10 + 1
      positions[player] = next_position
      scores[player] += next_position
      if scores[player] >= 1000:
        finished = True
        break
    if finished:
      break
  losing_score = min(scores)
  print(
    "PART 1: Score of losing player x num rolls of dice is "
    f"{losing_score} x {rolls} = {losing_score * rolls}"
  )
  

def part_2():
  """Complex part 2."""
  positions = _load_starting_positions()
  states = [
    {(positions[0], 0): 1},
    {(positions[1], 0): 1},
  ]
  wins = [0, 0]
  while states[0] or states[1]:
    for player, other in [(0, 1), (1, 0)]:
      n_other_players_still_playing = sum(states[other].values())
      current_states = states[player]
      next_states = defaultdict(int)
      for roll, possibilities in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]:
        for (pos, score), n_players_in_current_state in current_states.items():
          next_pos = (pos + roll - 1) % 10 + 1
          next_score = score + next_pos
          n_players_in_next_state = n_players_in_current_state * possibilities
          if next_score >= 21:
            wins[player] += n_players_in_next_state * n_other_players_still_playing
            continue
          next_states[(next_pos, next_score)] += n_players_in_next_state
      states[player] = next_states
  print(f"PART 2: Player that wins the most times wins in {max(wins)} universes!")


def _load_starting_positions():
  input_file = Path(__file__).parent.parent / "data" / "day21.txt"
  with input_file.open() as f:
    return [int(line.strip().split()[-1]) for line in f]


if __name__ == "__main__":
  part_1()
  part_2()
