from dataclasses import dataclass, field
from enum import Enum
from itertools import chain
from pathlib import Path
from typing import Any, cast

import typer


class NodeType(Enum):
    DIR = 0
    FILE = 1


@dataclass
class Node:
    name: str
    node_type: NodeType
    file_size: int = 0
    parent: Any = None
    children: list[Any] = field(default_factory=list)

    def node_size(self) -> int:
        return self.file_size + sum(map(Node.node_size, self.children))

    def all_children(self):
        return self.children + list(chain(*[c.all_children() for c in self.children]))


app = typer.Typer()


def read_input_file(input_file_path):
    p = Path(input_file_path)

    with p.open() as f:
        lines = f.readlines()

    return list(map(lambda l: l.strip(), lines))


def find_node(name, node_list) -> Node:
    return list(filter(lambda n: n.name == name, node_list))[0]


def process_commands(commands) -> Node:
    root = Node("/", NodeType.DIR)

    current_node = root

    for c in commands[2:]:
        parts = c.split()
        if parts[0] == "dir":
            cast(Node, current_node).children.append(
                Node(parts[1], NodeType.DIR, parent=current_node)
            )
        if parts[0].isdigit():
            cast(Node, current_node).children.append(
                Node(
                    parts[1],
                    NodeType.FILE,
                    file_size=int(parts[0]),
                    parent=current_node,
                )
            )
        if parts[0] == "$" and parts[1] == "cd" and parts[2] != "..":
            current_node = find_node(parts[2], cast(Node, current_node).children)
        if parts[0] == "$" and parts[1] == "cd" and parts[2] == "..":
            current_node = cast(Node, current_node).parent

    return root


def solve_part_1(commands) -> int:
    tree = process_commands(commands)
    all_nodes = tree.all_children()
    result = sum(
        filter(
            lambda s: s <= 100000,
            map(
                Node.node_size, filter(lambda n: n.node_type == NodeType.DIR, all_nodes)
            ),
        )
    )
    return result


def solve_part_2(commands) -> int:
    tree = process_commands(commands)
    size_root = tree.node_size()
    required_space = 30000000 - (70000000 - size_root)
    all_nodes = tree.all_children()
    result = min(
        filter(
            lambda s: s >= required_space,
            map(
                Node.node_size, filter(lambda n: n.node_type == NodeType.DIR, all_nodes)
            ),
        )
    )
    return result


@app.command()
def part1(input_file: str):
    commands = read_input_file(input_file)
    result = solve_part_1(commands)

    print(f"The sum of the total sizes of the directories < 100000 is: {result}")


@app.command()
def part2(input_file: str):
    commands = read_input_file(input_file)
    result = solve_part_2(commands)

    print(f"The size of the smallest directory freeing enough space is: {result}")


if __name__ == "__main__":
    app()
