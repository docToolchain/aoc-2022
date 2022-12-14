import regex as re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    input_pattern = re.compile("(\d+)")

    rocks = set()
    for line in lines_stripped:
        ends = [int(coord) for coord in (re.findall(input_pattern,line))]
        index = 0
        while index != len(ends)-2:
            if ends[index] == ends[index+2]:
                if ends[index+1] <= ends[index+3]:
                    rocks.update([(ends[index],j) for j in list(range(ends[index+1],ends[index+3]+1))])
                else:
                    rocks.update([(ends[index],j) for j in list(range(ends[index+1],ends[index+3]-1,-1))])
            else:
                if ends[index] <= ends[index+2]:
                    rocks.update([(j,ends[index+1]) for j in list(range(ends[index],ends[index+2]+1))])
                else:
                    rocks.update([(j,ends[index+1]) for j in list(range(ends[index],ends[index+2]-1,-1))])
            index += 2
            
    return rocks

def falling_sand(sand,deepest_point,rocks):
    while sand[1] < deepest_point:
        y = sand[1]
        while ((sand[0],y) not in rocks) and y<deepest_point+1:
            y += 1
        sand = (sand[0],y)
        if y == deepest_point+1:
            return (sand[0],y-1)
        if (sand[0]-1,sand[1]) in rocks:
            if (sand[0]+1,sand[1]) in rocks:
                return (sand[0],sand[1]-1)
            else:
                sand = (sand[0]+1,sand[1])
        else:
            sand = (sand[0]-1,sand[1])
        
    return sand

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    #star 1
    rocks = process_input(input_lines)

    deepest_point = max([rock[1] for rock in list(rocks)])

    unit_no = 0
    sand = (500,0)
    while sand[1] < deepest_point:
        sand = (500,0)
        sand = falling_sand(sand,deepest_point,rocks)
        rocks.add(sand)
        unit_no += 1

    print(unit_no-1)

    #star 2
    rocks = process_input(input_lines)

    deepest_point = max([rock[1] for rock in list(rocks)]) + 1
    
    unit_no = 0
    sand = (500,0)
    
    while (500,0) not in rocks:
        sand = (500,0)
        sand = falling_sand(sand,deepest_point,rocks)
        rocks.add(sand)
        unit_no += 1
        
    print(unit_no)

main()
