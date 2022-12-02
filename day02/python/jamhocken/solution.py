def process_input(file_contents):
    strategy = [line.strip() for line in file_contents]

    return strategy

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    strategy = process_input(input_lines)

    #star 1
    score_dict = {
        "A X": 1+3,
        "A Y": 2+6,
        "A Z": 3+0,
        "B X": 1+0,
        "B Y": 2+3,
        "B Z": 3+6,
        "C X": 1+6,
        "C Y": 2+0,
        "C Z": 3+3
    }

    scores = [score_dict[x] for x in strategy]
    print("The total score is",sum(scores))
    
    #star 2
    score_dict = {
        "A X": 3+0,
        "A Y": 1+3,
        "A Z": 2+6,
        "B X": 1+0,
        "B Y": 2+3,
        "B Z": 3+6,
        "C X": 2+0,
        "C Y": 3+3,
        "C Z": 1+6
    }

    scores = [score_dict[x] for x in strategy]
    print("The total score is",sum(scores))

main()
