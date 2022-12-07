#!/usr/bin/env python3

import networkx as nx


def load_input():
    data = nx.DiGraph()
    cur_dir = ''
    with open('input') as fd:
        for line in fd:
            temp = line.strip().split()
            if temp[0] == '$' and temp[1] == 'cd':
                if temp[2] == '..':
                    preds = list(data.predecessors(cur_dir))
                    cur_dir = preds[0]
                else:
                    cur_dir = cur_dir + temp[2]
                if not cur_dir.endswith('/'):
                    cur_dir += '/'
                data.add_node(cur_dir)
            elif temp[0] == '$' and temp[1] == 'ls':
                pass
            elif temp[0] == 'dir':
                data.add_edge(cur_dir, cur_dir + temp[1] + '/')
            else:
                size = int(temp[0])
                filename = temp[1]
                data.nodes[cur_dir][filename] = size
    return data


def get_total_size(g, node):
    total = sum(g.nodes[node].values())
    for successor in g.successors(node):
        total += get_total_size(g, successor)
    g.nodes[node]['total'] = total
    return total


def part1():
    data = load_input()
    total = get_total_size(data, '/')

    result = 0
    for node in data.nodes:
        total = data.nodes[node]['total']
        if total <= 100000:
            result += total

    print("Result 1: ", result)


def part2():
    data = load_input()
    total = get_total_size(data, '/')

    needed_space = 30000000 - (70000000 - total)
    temp = [data.nodes[node]['total'] for node in data.nodes if data.nodes[node]['total'] > needed_space]
    temp.sort()
    result = temp[0]
    print("Result 2: ", result)


def main():
    part1()
    part2()


if __name__ == '__main__':
    main()
