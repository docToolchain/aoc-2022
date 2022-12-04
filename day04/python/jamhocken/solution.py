import regex as re

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    input_pattern = re.compile("(\d+)-(\d+),(\d+)-(\d+)")
    sections=[]
    for line in lines_stripped:
        input_match = re.match(input_pattern, line)
        section = [int(input_match.group(i)) for i in [1, 2, 3,4]]
        sections.append(section)
    return sections

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    sections = process_input(input_lines)

    #star 1
    fully_contain = [1 if (pair[0]<=pair[2] and pair[1]>=pair[3]) or 
                     (pair[0]>=pair[2] and pair[1]<=pair[3]) else 0 
                     for pair in sections]
    print(sum(fully_contain),"pairs have one range fully contained by the other.")
    
    #star 2
    overlap = [1 if (pair[0]<=pair[2] and pair[1]>=pair[2]) or 
                     (pair[0]>=pair[2] and pair[0]<=pair[3]) else 0 
                     for pair in sections]
    print(sum(overlap),"pairs overlap.")
    
main()
