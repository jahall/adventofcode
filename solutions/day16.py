from dataclasses import dataclass
from pathlib import Path
from typing import List, Optional, Union

import numpy as np


@dataclass
class Packet:
  version: int
  type_id: int
  data: Optional[Union[List["Packet"], int]] = None

  @classmethod
  def from_message(cls,
      message: str,
      index: int = 0,
      return_index: bool = False,
      level: int = 0,
      verbose: bool = False,
    ):
    """Instantiate from message."""
    i = index
    packet = cls(
      version=cls._to_int(message[i:i + 3]),
      type_id=cls._to_int(message[i + 3:i + 6]),
    )
    i += 6
    # Literal
    if packet.type_id == 4:
      num_repr = ""
      while True:
        prefix = message[i]
        num_repr += message[i + 1:i + 5]
        i += 5
        if prefix == "0":
          break
      packet.data = cls._to_int(num_repr)
      if verbose:
        print(f"{'-' * level} {packet}")
    # Operator
    else:
      if verbose:
        print(f"{'-' * level} {packet}")
      length_type_id = message[i]
      i += 1
      offset = 15 if length_type_id == "0" else 11
      length = cls._to_int(message[i:i + offset])
      i += offset
      start, sub_packets, count = i, [], 0
      while True:
        count += 1
        sub_packet, i = Packet.from_message(
          message,
          index=i,
          level=level + 1,
          return_index=True,
        )
        sub_packets.append(sub_packet)
        if length_type_id == "0" and (i - start) >= length:
          break
        elif length_type_id == "1" and count == length:
          break
      packet.data = sub_packets
    return (packet, i) if return_index else packet

  def eval(self):
    """Evaluate the expression."""
    if self.type_id == 4:
      return self.data
    operators = {
      0: sum,
      1: np.prod,
      2: min,
      3: max,
      5: lambda x: int(x[0] > x[1]),
      6: lambda x: int(x[0] < x[1]),
      7: lambda x: int(x[0] == x[1]),
    }
    sub_values = [sp.eval() for sp in self.data]
    return operators[self.type_id](sub_values)

  @staticmethod
  def _to_int(binary_str):
    # surely a better way than this :)
    num = 0
    for p, val in enumerate(reversed(binary_str)):
      num += int(val) * 2 ** p
    return num

  def version_sum(self):
    """Get nested sum of all versions."""
    version_sum = self.version
    if isinstance(self.data, list):
      version_sum += sum(sp.version_sum() for sp in self.data)
    return version_sum


def part_1():
  """Simple part 1"""
  packet = Packet.from_message(_load_message())
  print(f"PART 1: Sum of versions is: {packet.version_sum()}")


def part_2():
  """Complex part 2"""
  packet = Packet.from_message(_load_message())
  print(f"PART 2: Value of the packet is: {packet.eval()}")


def _load_message():
  to_binary = {
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
  }
  input_file = Path(__file__).parent.parent / "data" / "day16.txt"
  with input_file.open() as f:
    raw_message = next(f).strip()
  # Part 1 Examples
  #raw_message = "8A004A801A8002F478"
  #raw_message = "620080001611562C8802118E34"
  #raw_message = "C0015000016115A2E0802F182340"
  #raw_message = "A0016C880162017C3686B18A3D4780"
  # Part 2 Examples
  #raw_message = "CE00C43D881120"
  #raw_message = "9C0141080250320F1802104A08"
  message = "".join(to_binary[char] for char in raw_message)
  return message


if __name__ == "__main__":
  part_1()
  part_2()
