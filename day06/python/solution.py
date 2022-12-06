def check_window(input_lines,length):
    window = input_lines[0:length]
    position = length
    if len(set(window))==length:
        return position
    else:
        for letter in input_lines[length:]:
            position += 1
            window = input_lines[position - length:position]
            if len(set(window))==length:
                return position

    return

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()[0]

    #star 1
    position = check_window(input_lines,4)
    print(position)
            
    #star 2
    position = check_window(input_lines,14)
    print(position)

main()
