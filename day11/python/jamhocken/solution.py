import regex as re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    
    monkeys = dict()
    item_pattern = re.compile("(\d+)")
    
    for monkey in range((len(lines_stripped)+1)//7):
        first_line = monkey*7
        items = [int(item) for item in re.findall(item_pattern, lines_stripped[first_line+1])]
        operation = (lines_stripped[first_line+2][21],lines_stripped[first_line+2][23:])
        test = int(re.findall(item_pattern, lines_stripped[first_line+3])[0])
        is_true = int(re.findall(item_pattern, lines_stripped[first_line+4])[0])
        is_false = int(re.findall(item_pattern, lines_stripped[first_line+5])[0])
        
        monkeys[int(lines_stripped[first_line][7])] = [items,operation,test,is_true,is_false,0]
    
    return monkeys

def process_monkey(monkeys,monkey,worry_divider):
    item_no = len(monkeys[monkey][0])
    for item in monkeys[monkey][0]:
        if monkeys[monkey][1][0] == '*':
            if monkeys[monkey][1][1].isdigit():
                new_worry = item*int(monkeys[monkey][1][1])
            else:
                new_worry = item*item
        else:
            if monkeys[monkey][1][1].isdigit():
                new_worry = item+int(monkeys[monkey][1][1])
            else:
                new_worry = item+item
        new_worry = new_worry // worry_divider
        if new_worry % monkeys[monkey][2] == 0:
            monkeys[monkeys[monkey][3]][0].append(new_worry)
        else:
            monkeys[monkeys[monkey][4]][0].append(new_worry)
            
    monkeys[monkey][0] = list()
    monkeys[monkey][5] += item_no
    
    return

def process_monkey2(monkeys,monkey,items):
    item_no = len(monkeys[monkey][0])
    for item in monkeys[monkey][0]:
        if monkeys[monkey][1][0] == '*':
            if monkeys[monkey][1][1].isdigit():
                items[item] = [(items[item][i]*int(monkeys[monkey][1][1]))%monkeys[i][2] for i in range(len(monkeys))]
            else:
                items[item] = [(items[item][i]**2)%monkeys[i][2] for i in range(len(monkeys))]
        else:
            if monkeys[monkey][1][1].isdigit():
                items[item] = [(items[item][i]+int(monkeys[monkey][1][1]))%monkeys[i][2] for i in range(len(monkeys))]
            else:
                items[item] = [(items[item][i]*2)%monkeys[i][2] for i in range(len(monkeys))]
        if items[item][monkey] == 0:
            monkeys[monkeys[monkey][3]][0].append(item)
        else:
            monkeys[monkeys[monkey][4]][0].append(item)
            
    monkeys[monkey][0] = list()
    monkeys[monkey][5] += item_no
    
    return

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()
    
    # star 1
    monkeys = process_input(input_lines)
    
    for i in range(20):
        for monkey in monkeys:
            process_monkey(monkeys,monkey,3)
        
    inspections = [value[5] for value in monkeys.values()]
    inspections.sort(reverse=True)
    print(inspections[0]*inspections[1])
    
    # star 2
    monkeys = process_input(input_lines)
    
    index = 0
    items = dict()
    for monkey in monkeys:
        for i,item in enumerate(monkeys[monkey][0]):
            items[index] = [item % m[2] for m in monkeys.values()]
            monkeys[monkey][0][i] = index
            index += 1
 
    for i in range(10000):
        for monkey in monkeys:
            process_monkey2(monkeys,monkey,items)
    
    inspections = [value[5] for value in monkeys.values()]
    inspections.sort(reverse=True)
    print(inspections[0]*inspections[1])
        
main()
