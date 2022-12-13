def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    nodes = list()
    line_length = len(lines_stripped[0])
    no_rows = len(lines_stripped)
    for i,line in enumerate(lines_stripped):
        for j,letter in enumerate(line):
            adjacent = set()
            index = i*line_length+j
            if i != 0:
                adjacent.add(index-line_length)
            if i != no_rows-1:
                adjacent.add(index+line_length)
            if index % line_length != 0:
                adjacent.add(index-1)
            if (index+1) % (line_length) != 0:
                adjacent.add(index+1)
            if letter == "S":
                distance = 0
            else:
                distance = float('Inf')
            nodes.append([index,letter,distance,adjacent,0])

    for node in range(len(nodes)):
        temp_neighbors = nodes[node][3].copy()
        for neighbor in nodes[node][3]:
            if nodes[neighbor][1] == "S":
                temp_neighbor = "a"
            elif nodes[neighbor][1] == "E":
                temp_neighbor = "z"
            else:
                temp_neighbor = nodes[neighbor][1]
            if nodes[node][1] == "S":
                temp_node = "a"
            elif nodes[node][1] == "E":
                temp_node = "z"
            else:
                temp_node = nodes[node][1]
            if ord(temp_neighbor) > ord(temp_node)+1:
                temp_neighbors.remove(neighbor)
        nodes[node][3] = temp_neighbors

    return nodes

def find_shortest_path(nodes,current_node):
    visited = set()

    valued = {current_node}
    while nodes[current_node][1] != 'E':
        neighbors = nodes[current_node][3]
        valued.update(neighbors-visited)
        for neighbor in neighbors:
            if neighbor not in visited:
                if nodes[neighbor][2] > nodes[current_node][2]+1:
                    nodes[neighbor][2] = nodes[current_node][2]+1
                    nodes[neighbor][4] = current_node

        visited.add(current_node)
        valued.remove(current_node)
        
        if valued != set():
            min_distance = min([nodes[node][2] for node in valued])
        else:
            return float('Inf')

        for node in valued:
            if nodes[node][2] == min_distance:
                current_node = node

    risk = nodes[current_node][2]

    return risk


def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    #star 1
    nodes = process_input(input_lines)

    for node in nodes:
        if node[1] == 'S':
            current_node = node[0]
            node_S = node[0]

    risk = find_shortest_path(nodes,current_node)

    print(risk)

    #star 2
    
    for node in nodes:
        if node[1] == "a":
            nodes = process_input(input_lines)
            nodes[node_S][2] = float('Inf')
            nodes[node[0]][2] = 0
            temp_risk = find_shortest_path(nodes,node[0])
            if temp_risk < risk:
                risk = temp_risk

    print(risk)

main()
