from collections import deque
from enum import Enum
from functools import reduce
import itertools
import operator


class PulseType(Enum):
    Off = 0
    Low = 1
    High = 2


class Node:
    def __init__(self, name: str, destinations: list[str]):
        self.name = name
        self.destinations = destinations

    def pulse(self, source: str, pulse_type: PulseType) -> PulseType:
        raise NotImplementedError()


class FlipFlop(Node):
    def __init__(self, name: str, destinations: list[str]):
        super().__init__(name, destinations)
        self.on = False

    def pulse(self, source: str, pulse_type: PulseType) -> PulseType:
        if pulse_type == PulseType.Low:
            if self.on:
                self.on = False
                return PulseType.Low
            else:
                self.on = True
                return PulseType.High
        else:
            return PulseType.Off


class Conjunction(Node):
    def __init__(self, name: str, destinations: list[str]):
        super().__init__(name, destinations)
        self.inputs = {}

    def pulse(self, source: str, pulse_type: PulseType) -> PulseType:
        if pulse_type != PulseType.Off:
            self.inputs[source] = pulse_type
            if all(pulse_type == PulseType.High for pulse_type in self.inputs.values()):
                return PulseType.Low
            else:
                return PulseType.High
        else:
            return PulseType.Off


class Source(Node):
    def pulse(self, source: str, pulse_type: PulseType) -> PulseType:
        return PulseType.Low


class Sink(Node):
    def pulse(self, source: str, pulse_type: PulseType) -> PulseType:
        return PulseType.Off


def setup_nodes(input: str) -> dict[str, Node]:
    nodes = {}

    for line in input:
        node_name, destinations = line.split("->")
        dest_nodes = [dest.strip() for dest in destinations.split(",")]
        node_name = node_name.strip()
        if node_name.startswith("&"):
            node = Conjunction(node_name[1:], dest_nodes)
        elif node_name.startswith("%"):
            node = FlipFlop(node_name[1:], dest_nodes)
        else:
            node = Source(node_name, dest_nodes)

        nodes[node.name] = node

    # set inputs for conjunction nodes
    for node in nodes.values():
        if isinstance(node, Conjunction):
            input_nodes = [
                n.name for n in nodes.values() if node.name in n.destinations
            ]
            for input in input_nodes:
                node.inputs[input] = PulseType.Low

    # find sink nodes
    all_destinations = set(
        itertools.chain.from_iterable(node.destinations for node in nodes.values())
    )
    sink_nodes = all_destinations.difference(set(nodes.keys()))
    for sink in sink_nodes:
        nodes[sink] = Sink(sink, [])

    return nodes


def part1(input: str) -> int:
    nodes = setup_nodes(input)

    low_pulses = 0
    high_pulses = 0
    button_presses = 1000

    for _ in range(button_presses):
        queue = deque([("broadcaster", "button", PulseType.Low)])
        while queue:
            node_name, source, pulse_type = queue.popleft()

            if pulse_type == PulseType.Low:
                low_pulses += 1
            elif pulse_type == PulseType.High:
                high_pulses += 1

            node = nodes[node_name]
            new_pulse = node.pulse(source, pulse_type)
            if new_pulse != PulseType.Off:
                for dest in node.destinations:
                    queue.append((dest, node_name, new_pulse))

    return low_pulses * high_pulses


def part2(input: str) -> int:
    nodes = setup_nodes(input)

    button_presses = 1000000

    # Low pulse is sent to final node "rx" when the
    # conjunction node connected to it send a low pulse.
    # This happens when the nodes connected to that node send a high pulse.

    second_last_node = next(
        node.name for node in nodes.values() if "rx" in node.destinations
    )
    conjuction_nodes = [
        name for (name, node) in nodes.items() if second_last_node in node.destinations
    ]

    seen = {}
    cycle_lengths = {}

    for idx in range(button_presses):
        queue = deque([("broadcaster", "button", PulseType.Low)])
        while queue:
            node_name, source, pulse_type = queue.popleft()

            node = nodes[node_name]
            new_pulse = node.pulse(source, pulse_type)
            if (
                isinstance(node, Conjunction)
                and new_pulse == PulseType.High
                and node_name in conjuction_nodes
            ):
                if node_name not in seen:
                    seen[node_name] = idx
                else:
                    length = idx - seen[node_name]
                    print(f"Cycle length {length} for node {node_name}")
                    cycle_lengths[node_name] = length
                    seen[node_name] = idx
                    if all(name in cycle_lengths for name in conjuction_nodes):
                        lcm = reduce(operator.mul, cycle_lengths.values(), 1)
                        return lcm
            if new_pulse != PulseType.Off:
                for dest in node.destinations:
                    queue.append((dest, node_name, new_pulse))


if __name__ == "__main__":
    with open("input.txt") as f:
        inp = f.readlines()

    print(part1(inp))
    print(part2(inp))
