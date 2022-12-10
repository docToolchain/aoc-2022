def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    moves = [((1,0),int(line[2:])) if line[0] == "R" \
             else ((-1,0),int(line[2:])) if line[0] == "L" \
                   else ((0,1),int(line[2:])) if line[0] == "U" \
                       else ((0,-1),int(line[2:])) \
                           for line in lines_stripped]
              
    return moves

def move_ropes(move,current_h,current_t):

    current_h[0] += move[0][0]
    current_h[1] += move[0][1]
    difference = [current_h[0] - current_t[0],current_h[1] - current_t[1]]
    if difference in [[0,0],[0,1],[0,-1],[1,0],[1,1],[1,-1],[-1,0],[-1,1],[-1,-1]]:
        pass
    else:
        if difference[1] >= 1:
            current_t[1] += 1
        elif difference[1] <= -1:
            current_t[1] -= 1
        if difference[0] >= 1:
            current_t[0] += 1
        elif difference[0] <= -1:
            current_t[0] -= 1

    return current_h, current_t, tuple(current_t)

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    moves = process_input(input_lines)
    
    # star 1
    positions = set([(0,0)])
    current_h = [0,0]
    current_t = [0,0]

    for move in moves:
        for index in range(move[1]):
            current_h, current_t,positions_temp = move_ropes(move, current_h, current_t)
            positions.add(positions_temp)
    print(len(positions))
            
    # star 2
    positions = set([(0,0)])
    current_rope = [[0,0] for index in range(10)]
    for move in moves:
        for index1 in range(move[1]):
            move_temp = move
            for index in range(9):
                current_h = current_rope[index]
                current_t = current_rope[index+1]
                current_h, current_t,positions_temp = move_ropes(move_temp, current_h, current_t)
                move_temp = ((current_t[0]-current_rope[index+1][0],current_t[1]-current_rope[index+1][1]),move[1])
                current_rope[index] = current_h
            positions.add(positions_temp)
    
    print(len(positions))
    
main()
