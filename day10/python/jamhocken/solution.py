def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    instructions = [(1,0) if inst[0]=="n" else (2,int(inst[5:])) for inst in lines_stripped]
    
    return instructions

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    instructions = process_input(input_lines)
    
    # star 1
    cpu = list()
    cpu.append([0,1])
    values = list()

    for index,inst in enumerate(instructions):
        cpu.append([cpu[index][0]+inst[0],cpu[index][1]+inst[1]])
        if cpu[index+1][0] >= 20 + len(values)*40:
            values.append(cpu[index][1])

    print(sum([value*(20+40*index) for index,value in enumerate(values)]))
        
    # star 2
    sprite = {c[0]:[c[1]-1,c[1],c[1]+1] for c in cpu}
    keys = sprite.keys()
    for i in range(240):
        if i in keys:
            current_sprite = sprite[i]
        else:
            sprite[i] = current_sprite
 
    for y in range(6):
        for x in range(40):
            if x in sprite[x+y*40]:
                print("#",end='')
            else:
                print(".",end='')
        print("")
        
main()
