#!/usr/bin/env python3


import networkx as nx
import itertools as it

import re
from copy import deepcopy


limit_combos = dict()


def load_input():
    data = list()
    with open('input') as fd:
        pattern = re.compile('Valve (\w+) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? (.+)')
        for line in fd:
            match = pattern.search(line.strip())
            valve = match.group(1)
            rate = int(match.group(2))
            neighbours = list(map(lambda x: x.strip(), match.group(3).split(',')))
            data.append((valve, rate, neighbours))
    return data


def create_power_set(data):
    result = dict()
    for i in range(1, len(data)):
        for combo in it.combinations(data, i):
            key = tuple(sorted(combo))
            result[key] = [0, 0]
    return result


def create_graph(data):
    graph = nx.Graph()
    for item in data:
        node = item[0]
        graph.add_node(node, flow=item[1], w=0, wm=-1, valves=set())
        for neighbour in item[2]:
            graph.add_edge(node, neighbour)

    return graph


def shortest_valves(graph, start):
    shortest_valves = dict()
    valves = [node for node, values in graph.nodes.items() if values['flow'] != 0]
    valves.append(start)
    for valve in valves:
        shortest_valves[valve] = dict()
        for nv in valves:
            if nv != valve:
                shortest_valves[valve][nv] = nx.shortest_path_length(graph, valve, nv)
    return shortest_valves


def process_part1(nodes, valves, node, pressure, time, remaining, path=list(), depth=0):
    cycles = 30

    if path:
        path.sort()
        key = tuple(path)
        if pressure <= limit_combos[key][0] and time >= limit_combos[key][1]:
            return path, pressure
        else:
            limit_combos[key][0] = pressure
            limit_combos[key][1] = time

    path.append(node)
    path.sort()

    max_pressure = pressure
    max_path = path.copy()

    for valve in remaining:
        length = valves[node][valve]
        tt = time + length + 1
        if tt >= cycles:
            continue
        tr = remaining.copy()
        tr.remove(valve)
        tp = pressure + nodes[valve]['flow']*(cycles-tt)
        tpath, tpressure = process_part1(nodes, valves, valve, tp, tt, tr, path.copy(), depth+1)
        if tpressure > max_pressure:
            max_pressure = tpressure
            max_path = tpath
    return max_path, max_pressure


def process_part2(nodes, valves, node, pressure, time, remaining, path=set(), depth=0):
    cycles = 26

    if path:
        temp = list(path)
        temp.sort()
        key = tuple(temp)
        if pressure <= limit_combos[key][0] and max(time) >= limit_combos[key][1]:
            return path, pressure
        else:
            limit_combos[key][0] = pressure
            limit_combos[key][1] = max(time)

    path.update(set(node))

    max_pressure = pressure
    max_path = path.copy()

    active = 1 if time[0] > time[1] else 0

    for valve in remaining:
        length = valves[node[active]][valve]

        tt = deepcopy(time)
        tt[active] = time[active] + length + 1

        if tt[active] >= cycles:
            continue

        tr = remaining.copy()
        tr.remove(valve)

        tn = node.copy()
        tn[active] = valve

        tp = pressure + nodes[valve]['flow']*(cycles-tt[active])
        tpath, tpressure = process_part2(nodes, valves, tn, tp, tt, tr, path.copy(), depth + 1)

        if tpressure > max_pressure:
            max_pressure = tpressure
            max_path = tpath

    return max_path, max_pressure


def part1():
    data = load_input()
    graph = create_graph(data)

    start = 'AA'
    valves = shortest_valves(graph, start)

    global limit_combos
    limit_combos = create_power_set(list(valves.keys()))

    remaining = set(valves.keys())
    remaining.remove('AA')

    path, pressure = process_part1(graph.nodes, valves, start, 0, 0, remaining)
    result = pressure

    print("Result 1: ", result)


def part2():
    data = load_input()
    data = load_input()
    graph = create_graph(data)

    start = ['AA', 'AA']
    valves = shortest_valves(graph, start[0])

    global limit_combos
    limit_combos = create_power_set(list(valves.keys()))

    remaining = set(valves.keys())
    remaining.remove('AA')

    path, pressure = process_part2(graph.nodes, valves, start, 0, [0, 0], remaining)

    result = pressure
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
