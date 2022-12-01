def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    cal = 0
    calories = []
    for line in lines_stripped:
        if not line:
            calories.append(cal)
            cal = 0
        else:
            cal += int(line)

    return calories

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    calories = process_input(input_lines)

    #star 1
    print("The elf with the most calories is carrying",max(calories),"calories.")
    
    #star 2
    calories.sort(reverse=True)
    print("The top three elves are carrying",sum(calories[0:3]),"calories.")

main()
