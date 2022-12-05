import regex as re
from collections import deque

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    for index,line in enumerate(lines_stripped):
        if line[0:1] == "1":
            stack_height = index
            no_stacks = int(line[-1])
            break
    
    stacks = {index:deque() for index in range(no_stacks)}
    for stack in range(no_stacks):
        for position in range(stack_height):
            if file_contents[stack_height-position-1][(stack)*4+1] != ' ':
                stacks[stack].append(file_contents[stack_height-position-1][stack*4+1])
            else:
                break
    
    input_pattern = re.compile("move (\d+) from (\d+) to (\d+)")
    instructions = []
    for line_no in range(stack_height+2,len(lines_stripped)):
        input_match = re.match(input_pattern, lines_stripped[line_no])
        instruction = [int(input_match.group(i)) for i in [1, 2, 3]]
        instructions.append(instruction)

    return stacks, instructions

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    [stacks, instructions] = process_input(input_lines)

    #star 1
    for instruction in instructions:
        for crate_no in range(instruction[0]):
            stacks[instruction[2]-1].append(stacks[instruction[1]-1].pop())
    
    print(''.join([stacks[i].pop() for i in range(len(stacks))]))
    
    #star 2
    [stacks, instructions] = process_input(input_lines)
    
    for instruction in instructions:
        temp_stack = deque()
        for crate_no in range(instruction[0]):
            temp_stack.append(stacks[instruction[1]-1].pop())  
        for crate_no in range(len(temp_stack)):
            stacks[instruction[2]-1].append(temp_stack.pop())

    print(''.join([stacks[i].pop() for i in range(len(stacks))]))

main()
