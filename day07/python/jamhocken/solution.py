def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    commands = [line.split() for line in lines_stripped]

    return commands

def dir_size(parent_child, directory):
    if parent_child[directory][1] == 0:
        for item in parent_child[directory][2:]:
            if type(item) != tuple:
                parent_child[directory][1] = parent_child[directory][1] + item
            else:
                dir_size(parent_child,item)
                parent_child[directory][1] = parent_child[directory][1] + parent_child[item][1]
    return

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    commands = process_input(input_lines)

    current_directory = ("/",0)
    parent_child = {}
    parent = ("/",0)
    parent_child[parent] = [current_directory,0]
    keys_used = {"/":0}
    for command in commands:
        if command[0] == "$" and command[1] == "cd":
            if command[2] != ".." and command[2] != "/":
                parent = current_directory
                for value in parent_child[current_directory]:
                    if type(value) == tuple:
                        if value[0] == command[2]:
                            current_directory = value
            else:
                current_directory = parent
                parent = parent_child[current_directory][0]
        elif command[0] == "$" and command[1] == "ls":
            parent_child[current_directory] = [parent,0]
        else:
            if command[0] != "dir":
                parent_child[current_directory].append(int(command[0]))
            elif command[1] not in keys_used.keys():
                keys_used[command[1]] = 0
                parent_child[current_directory].append((command[1],0))
            else:
                keys_used[command[1]] = keys_used[command[1]]+1
                parent_child[current_directory].append((command[1],keys_used[command[1]]))

    dir_size(parent_child,("/",0))

    #star 1
    count = 0
    for value in parent_child.values():
        if value[1] <= 100000:
            count += value[1]

    print(count)

    #star 2
    space_free = 70000000 - parent_child[("/",0)][1]
    space_needed = 30000000 - space_free

    smallest = 70000000
    for value in parent_child.values():
        if value[1] > space_needed and value[1] < smallest:
            smallest = value[1]

    print(smallest)

main()
