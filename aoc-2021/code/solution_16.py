from pathlib import Path
from typing import List, Tuple


DATA_PATH = Path(__file__).resolve().parents[1] / "data"

CONVERSION = {
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


def read_input(path: Path) -> str:
    return path.read_text().strip("\n")


class Packet:
    def __init__(self, packet: str):
        self._packet = packet
        self.version = bits_to_int(packet[:3])
        self.type = bits_to_int(packet[3:6])
        self.decode()

    def decode(self):
        if self.type == 4:
            self.literal, self.stop = decode_literal(self._packet)
        else:
            self.packets = decode_operator(self._packet)


def decode(buffer: str):
    version, buffer = pop_until(buffer, 3)
    version = bits_to_int(version)
    type, buffer = pop_until(buffer, 3)
    data = {"version": version}
    if bits_to_int(type) == 4:
        literal, stop = decode_literal(buffer)
    else:
        packets = decode_operator(buffer, data)

    print(data)


def hex_to_binary(packet: str) -> str:
    return "".join(CONVERSION[c] for c in packet)


def bits_to_int(bits: str) -> int:
    out = 0
    for bit in bits:
        out = (out << 1) | int(bit)
    return out


def decode_literal(packet: str) -> Tuple[int, int]:
    literal = []
    for i in range(int(len(packet) / 5)):
        five_bits = packet[i * 5 : (i + 1) * 5]
        literal.extend(five_bits[1:])
        if five_bits[0] == "0":
            return bits_to_int("".join(literal)), 5 * i
    raise RuntimeError


def decode_operator(buffer: str, sum_version: dict) -> str:
    length_type_id, buffer = pop_until(buffer, 1)
    # print("type", length_type_id)
    if length_type_id == "0":
        total_length_bit, buffer = pop_until(buffer, 15)
        # print("total len", bits_to_int(total_length_bit), total_length_bit)
        buffer_to_decode, other = pop_until(buffer, bits_to_int(total_length_bit))
        decode_subpackets(buffer_to_decode, sum_version)
        return other
    elif length_type_id == "1":
        number_subpackets, buffer = pop_until(buffer, 11)
        decode_subpackets(buffer, sum_version, number=bits_to_int(number_subpackets))
        return ""
    else:
        raise ValueError


def pop_until(string: str, idx: int) -> Tuple[str, str]:
    return string[:idx], string[idx:]


def decode_subpackets(buffer: str, sum_version: dict, number=None):
    packets = []
    # print("num", number, buffer)
    while buffer:
        if len(packets) == number:
            break
        if not int(buffer):
            break
        version, buffer = pop_until(buffer, 3)
        sum_version["version"] += bits_to_int(version)
        type, buffer = pop_until(buffer, 3)
        if bits_to_int(type) == 4:
            literal = []
            i = 0
            while True:
                block, buffer = pop_until(buffer, 5)
                literal.extend(block[1:])
                if block[0] == "0":
                    break

            packets.append(bits_to_int("".join(literal)))
        else:
            buffer = decode_operator(buffer, sum_version)
    return


def main(problem_number: int):
    data = read_input(DATA_PATH / f"input_{problem_number}.txt")

    elem = "D2FE28"
    decode((hex_to_binary(elem)))

    elem = "38006F45291200"
    decode((hex_to_binary(elem)))

    elem = "EE00D40C823060"
    decode((hex_to_binary(elem)))

    elem = "8A004A801A8002F478"
    decode((hex_to_binary(elem)))

    elem = "620080001611562C8802118E34"
    decode((hex_to_binary(elem)))

    elem = "C0015000016115A2E0802F182340"
    decode((hex_to_binary(elem)))

    elem = "A0016C880162017C3686B18A3D4780"
    decode((hex_to_binary(elem)))
    # Part 1
    decode((hex_to_binary(data)))
