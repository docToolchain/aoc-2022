#!/usr/bin/env python3


import networkx as nx
import string
import math as m


def load_input():
    with open('input') as fd:
        data = list()
        start = None
        end = None
        for i, line in enumerate(fd):
            temp_line = list(line.strip())
            if 'S' in temp_line:
                j = temp_line.index('S')
                start = (i, j)
                temp_line[j] = 'a'
            if 'E' in temp_line:
                j = temp_line.index('E')
                end = (i, j)
                temp_line[j] = 'z'
            temp = [string.ascii_lowercase.index(v) for v in temp_line]
            data.append(temp)
    return data, start, end


def weight_func(n0, n1, attr):
    return 1 if attr['w'] <= 1 else None


def create_graph(data):
    graph = nx.DiGraph()
    for i in range(len(data)):
        for j in range(len(data[0])):
            if i+1 < len(data):
                graph.add_edge((i, j), (i+1, j), w=data[i+1][j] - data[i][j])
                graph.add_edge((i+1, j), (i, j), w=data[i][j] - data[i+1][j])
            if j+1 < len(data[0]):
                graph.add_edge((i, j), (i, j+1), w=data[i][j+1] - data[i][j])
                graph.add_edge((i, j+1), (i, j), w=data[i][j] - data[i][j+1])
    return graph


def part1():
    data, start, end = load_input()
    graph = create_graph(data)
    result = nx.shortest_path_length(graph, source=start, target=end, weight=weight_func)
    print("Result 1: ", result)


def part2():
    data, _, end = load_input()
    starts = list()
    shortest = list()
    for i in range(len(data)):
        for j in range(len(data[0])):
            if data[i][j] == 0:
                starts.append((i, j))

    graph = create_graph(data)

    for start in starts:
        try:
            temp = nx.shortest_path_length(graph, source=start, target=end, weight=weight_func)
            shortest.append(temp)
        except nx.exception.NetworkXNoPath:
            pass

    result = min(shortest)
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
