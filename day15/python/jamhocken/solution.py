import regex as re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    input_pattern = re.compile("(-*\d+)")

    sensor_beacons = list()
    for line in lines_stripped:
        next_sensors_beacons = [int(coord) for coord in (re.findall(input_pattern,line))]
        sensor_beacons.append(next_sensors_beacons)
            
    return sensor_beacons

def manhattan_distance(x,y):
    return abs(x[0]-y[0]) + abs(x[1]-y[1])

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    #star 1
    sensor_beacons = process_input(input_lines)
    
    count = 0
    y = 2000000

    positions = dict()
    for pair in sensor_beacons:
        if (pair[0],pair[1]) not in positions:
            positions[(pair[0],pair[1])] = "S"
            if pair[1] == y:
                count +=1
        if (pair[2],pair[3]) not in positions:
            positions[(pair[2],pair[3])] = "B"
       
    for pair in sensor_beacons:
        distance = abs(pair[0]-pair[2]) + abs(pair[1]-pair[3])
        if pair[1] > y:
            y_offset = pair[1] - y
        else:
            y_offset = y - pair[1]
        if y_offset <= distance:
            for x_offset in range(-1*distance+y_offset,distance-y_offset+1):
                if (pair[0]+x_offset,y) not in positions:
                    positions[(pair[0]+x_offset,y)] = "#"
                    count += 1
                    
    print(count)

    #star 2
    sensors_dist = dict()
    for pair in sensor_beacons:
        distance = manhattan_distance([pair[0],pair[1]],[pair[2],pair[3]])
        sensors_dist[(pair[0],pair[1])] = distance
    sensors = sensors_dist.keys()        
    
    search_field = 4000000

    for sensor in sensors:
        for x in range(max(0,-1*sensors_dist[sensor]-1+sensor[0]),min(sensors_dist[sensor]+2+sensor[0],search_field+1)):
            x_offset = x-sensor[0]
            for y_offset in [-1*sensors_dist[sensor]-1+abs(x_offset),sensors_dist[sensor]+1-abs(x_offset)]:
                if y_offset+sensor[1] < 0 or y_offset+sensor[1]>search_field:
                    pass
                else:
                    test = 1
                    for other_sensor in sensors:
                        if manhattan_distance((sensor[0]+x_offset,sensor[1]+y_offset),other_sensor)<sensors_dist[other_sensor]:
                            test = 0
                            break
                    if test == 1:
                        print((sensor[0]+x_offset)*4000000+(sensor[1]+y_offset))
                        break
            if test == 1:
                break
        if test == 1:
            break
            
main()
