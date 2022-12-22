import re
import copy

def main():
    file = open('input.txt', 'r')

    startingStacks, steps = file.read().split("\n\n")
    stacks = get_starting_stack(startingStacks)
    stacks_star2 = copy.deepcopy(stacks)

    steps = get_step_list(steps)
    
    stacks = process_steps_star1(stacks, steps)
    star_one(stacks)

    stacks = process_steps_star2(stacks_star2, steps)
    star_two(stacks)

def star_two(stacks):
    print("The solution for star 2 is: ")
    for stack in stacks:
        print(stack.pop())

def star_one(stacks):
    print("The solution for star 1 is: ")
    for stack in stacks:
        print(stack.pop())

def process_steps_star2(stacks, steps):
    for step in steps:
        crane = []     
        for cnt in range(step[0]):
            src, dest = step[1] - 1, step[2] - 1                
            crane.insert(0,stacks[src].pop())
        stacks[dest] = stacks[dest] + crane
      
    return stacks

def process_steps_star1(stacks, steps):
    for step in steps:
        for cnt in range(step[0]):
            src, dest = step[1] - 1, step[2] - 1
            stacks[dest].append(stacks[src].pop())

    return stacks

def get_starting_stack(startingStacks):
    startingStacks = startingStacks.splitlines()
    lastLine = startingStacks.pop()
    maxStackHeight = len(startingStacks)
    stackCount = int(lastLine.split().pop())

    stacks = []
    for column in range(stackCount):
        stacks.append([])
        offset = 1
        x = column * 4 + offset
        for row in reversed(range(maxStackHeight)):   
            symbol = startingStacks[row][x]
            if symbol != " ":
                stacks[column].append(symbol)

    return stacks

def get_step_list(steps): 
    stepList = []
    steps = steps.splitlines()
    for step in steps:
        result = re.search(r"move (\d*) from (\d*) to (\d*)", step)
        instructions = result.groups()
        instructions = [int(i) for i in instructions]
        stepList.append(instructions)

    return stepList    

if __name__ == '__main__':
    main()