def process_input(file_contents):
    rucksack = [line.strip() for line in file_contents]

    return rucksack

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    contents = process_input(input_lines)

    #star 1
    duplicates = []
    for rucksack in contents:
        for letter in rucksack[0:len(rucksack)//2]:
            if letter in rucksack[len(rucksack)//2:]:
                duplicates.append(letter)
                break

    scores = [ord(item)-96 if ord(item)>96 else ord(item)-64+26 for item in duplicates]
    print("The sum of the priorities is ",sum(scores))

    #star 2
    position = 0
    badge = []
    while position < len(contents):
        for letter in contents[position]:
            if letter in contents[position+1] and letter in contents[position+2]:
                badge.append(letter)
                position += 3
                break

    scores = [ord(item)-96 if ord(item)>96 else ord(item)-64+26 for item in badge]
    print("The sum of the priorities is ",sum(scores))

main()
