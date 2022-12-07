
file1 = open('input.txt', 'r')
Lines = file1.readlines()

def getItemValue(priority):
    if ord(priority) > 97:
        return ord(priority) - 96
    else:
        return ord(priority) - 38

priority_sum = 0
for line in Lines:
    characters = list(line.strip())
    middle = (int(len(characters) / 2)) 

    elf_one = characters[0 : middle ]
    elf_two = characters[middle : len(characters) ]

    item = list(set(elf_one) & set(elf_two)).pop()
    priority_sum += getItemValue(item)

print(f"Star 1: Item priority sum is {priority_sum}")

line_cnt = 0
groups = []
group = []

for line in Lines:
    group.append(list(line.strip()))
    line_cnt += 1
    if line_cnt == 3:     
        groups.append(group)
        group = []
        line_cnt = 0

priority_sum = 0
for group in groups:
    item =  list(set(group[0]) & set(group[1]) & set(group[2])).pop()
    priority_sum += getItemValue(item)

print(f"Star 2: Item priority sum is {priority_sum}")


