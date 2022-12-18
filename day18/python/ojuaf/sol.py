#!/usr/bin/env python3


import networkx as nx

import numpy as np


def load_input():
    data = set()
    with open('input') as fd:
        for line in fd:
            line = line.strip()
            data.add(tuple([int(i) for i in line.split(',')]))
    return data


def get_neighbors(field, point):
    x, y, z = point
    neighbors = []
    x_max, y_max, z_max = field.shape
    for i in range(-1, 2):
        for j in range(-1, 2):
            for k in range(-1, 2):
                if abs(i) + abs(j) + abs(k) == 1:
                    if x+i >= 0 and y+j >= 0 and z+k >= 0 and x+i < x_max and y+j < y_max and z+k < z_max:
                        neighbors.append((x+i, y+j, z+k))
    return neighbors


def parts():
    data = load_input()
    graph = nx.Graph()
    x_max = max((coord[0] for coord in data))
    y_max = max((coord[1] for coord in data))
    z_max = max((coord[2] for coord in data))
    cube = np.full((x_max+3, y_max+3, z_max+3), False, dtype=np.bool_)

    it = np.nditer(cube, flags=['multi_index'])

    for x, y, z in data:
        cube[x+1, y+1, z+1] = True

    side_count = 0
    it = np.nditer(cube, flags=['multi_index'])

    for val in it:
        x, y, z = it.multi_index
        
        if not val:
            graph.add_node((x, y, z))

        neighbors = get_neighbors(cube, (x, y, z))
        for i, j, k in neighbors:
            if not cube[i, j, k]:
                if val:
                    side_count += 1
                else:
                    graph.add_edge((x, y, z), (i, j, k))

    print("Result 1: ", side_count)

    # Part 2
    src = (0, 0, 0)
    tree = nx.bfs_tree(graph, src)

    lava_nodes = set(graph.nodes).difference(set(tree.nodes))

    side_count = 0
    it = np.nditer(cube, flags=['multi_index'])

    for val in it:
        x, y, z = it.multi_index
        if val:
            neighbors = get_neighbors(cube, (x, y, z))
            for i, j, k in neighbors:
                if not cube[i, j, k] and (i, j, k) not in lava_nodes:
                    side_count += 1

    print("Result 2: ", side_count)


def main():
    parts()


if __name__ == '__main__':
    main()
