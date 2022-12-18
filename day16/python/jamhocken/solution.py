import regex as re

def process_input(file_contents):

    valve_pattern = re.compile("Valve (..)")
    neighbor_pattern = re.compile("([A-Z][A-Z])[,\n]")
    rate_pattern = re.compile("rate=(\d+)")

    temp_valves = {re.match(valve_pattern,line).group(1):[int(re.findall(rate_pattern,line)[0]),re.findall(neighbor_pattern,line)] for line in file_contents}
    
    valve_index = {key:i for i,key in enumerate(temp_valves.keys())}
    valves = {valve_index[key]:[value[0],[valve_index[i] for i in value[1]]] for key,value in temp_valves.items()}
    
    position_AA = valve_index["AA"]
    
    return valves,position_AA

def find_max_flow(valves,position_state,t_max):
    t= 1

    pressure = 0
    while t < t_max+1:
        temp_position_state = position_state.copy()                    
         
        pressure = max(position_state.values())

#        print("Time = ", t)
#        print(pressure)

        max_pressure = [valves[i][0]*(t_max-t-1) for i in range(len(valves))]
        for current_state in temp_position_state:
            find_neighbors(valves,current_state,position_state,t_max-t,t+1,max_pressure,pressure)

        temp_position_state = position_state.copy()
        position_state = dict()
        for current_state in temp_position_state:
            if current_state[2] == t+1:
                if sum([0 if current_state[1][i] == True else max_pressure[i] for i in range(len(valves))]) + temp_position_state[current_state] >= pressure:
                    position_state[current_state] = temp_position_state[current_state]
        
#        print(len(position_state))
        
        t += 1

    return pressure

def find_neighbors(valves,current_state,position_state,time_left,t,max_pressure,pressure):
    if type(current_state[0]) is not tuple:
        neighbors = set()
        current_pressure = position_state[current_state]
        neighbors.update(tuple([tuple([valves[current_state[0]][1][i],current_state[1],t]) for i in range(len(valves[current_state[0]][1]))]))
        for neighbor in neighbors:
            if sum([0 if neighbor[1][i] == True else max_pressure[i] for i in range(len(valves))]) + current_pressure >= pressure:
                if neighbor not in position_state:
                    position_state[neighbor] = current_pressure
                elif position_state[neighbor] < current_pressure:
                    position_state[neighbor] = current_pressure
        if current_state[1][current_state[0]] == False and valves[current_state[0]][0] != 0:
            if tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0] else True for i in range(len(current_state[1]))]),t]) not in position_state:
                position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0] else True for i in range(len(current_state[1]))]),t])]\
                = current_pressure + time_left*valves[current_state[0]][0]
            elif position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0] else True for i in range(len(current_state[1]))]),t])] \
                < current_pressure + time_left*valves[current_state[0]][0]:
                position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0] else True for i in range(len(current_state[1]))]),t])] = current_pressure + time_left*valves[current_state[0]][0]
    else:
        # The first position moves or opens if possible
        neighbors = set()
        invalid_neighbor = set()
        current_pressure = position_state[current_state]
        neighbors.update(tuple([tuple([(valves[current_state[0][0]][1][i],current_state[0][1]),current_state[1],t]) for i in range(len(valves[current_state[0][0]][1]))]))
        for neighbor in neighbors:
            if sum([0 if neighbor[1][i] == True else max_pressure[i] for i in range(len(valves))]) + current_pressure >= pressure:
                if neighbor not in position_state:
                    position_state[neighbor] = current_pressure
                elif position_state[neighbor] < current_pressure:
                    position_state[neighbor] = current_pressure
            else:
                invalid_neighbor.add(neighbor)
        neighbors = neighbors - invalid_neighbor
        if current_state[1][current_state[0][0]] == False and valves[current_state[0][0]][0] != 0:
            if tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0][0] else True for i in range(len(current_state[1]))]),t]) not in position_state:
                neighbors.add(tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0][0] else True for i in range(len(current_state[1]))]),t]))
                position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0][0] else True for i in range(len(current_state[1]))]),t])]\
                = current_pressure + time_left*valves[current_state[0][0]][0]
            elif position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0][0] else True for i in range(len(current_state[1]))]),t])] \
                < current_pressure + time_left*valves[current_state[0][0]][0]:
                position_state[tuple([current_state[0],tuple([current_state[1][i] if i!= current_state[0][0] else True for i in range(len(current_state[1]))]),t])] = current_pressure + time_left*valves[current_state[0][0]][0]
        # And the second one (elephant or me) does its moving or opening
        only1move = neighbors.copy()
        for state in only1move:
            neighbors = set()
            current_pressure = position_state[state]
            neighbors.update(tuple([tuple([(state[0][0],valves[state[0][1]][1][i]),state[1],t]) for i in range(len(valves[state[0][1]][1]))]))
            for neighbor in neighbors:
                if neighbor not in position_state:
                    position_state[neighbor] = current_pressure
                elif position_state[neighbor] < current_pressure:
                    position_state[neighbor] = current_pressure
            if state[1][state[0][1]] == False and valves[state[0][1]][0] != 0:
                if tuple([state[0],tuple([state[1][i] if i!= state[0][1] else True for i in range(len(state[1]))]),t]) not in position_state:
                    neighbors.add(tuple([state[0],tuple([state[1][i] if i!= state[0][1] else True for i in range(len(state[1]))]),t]))
                    position_state[tuple([state[0],tuple([state[1][i] if i!= state[0][1] else True for i in range(len(state[1]))]),t])]\
                    = current_pressure + time_left*valves[state[0][1]][0]
                elif position_state[tuple([state[0],tuple([state[1][i] if i!= state[0][1] else True for i in range(len(state[1]))]),t])] \
                    < current_pressure + time_left*valves[state[0][1]][0]:
                    position_state[tuple([state[0],tuple([state[1][i] if i!= state[0][1] else True for i in range(len(current_state[1]))]),t])] = current_pressure + time_left*valves[state[0][1]][0]

    return

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()
        input_lines[-1] = input_lines[-1] + "\n"

    valves,position_AA = process_input(input_lines)
    
    #star 1

    start_position = (position_AA,tuple([False for i in range(len(valves))]),0)
    position_state = {start_position:0}
    t_max = 30
    
    pressure = find_max_flow(valves,position_state,t_max)
    
    print(pressure)
    
    #star 2
    
    start_position = ((position_AA,position_AA),tuple([False for i in range(len(valves))]),0)
    position_state = {start_position:0}
    t_max = 26

    pressure = find_max_flow(valves,position_state,t_max)
    
    print(pressure)
             
main()
