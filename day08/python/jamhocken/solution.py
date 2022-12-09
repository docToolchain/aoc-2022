import numpy as np

def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]
    trees = np.array([[int(entry) for entry in line] for line in lines_stripped])
    
    return trees

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    trees = process_input(input_lines)

    # star 1
    visible = (len(trees)-1)*4
    for row_no,row in enumerate(trees):
        if row_no != 0 and row_no != len(trees)-1:
            for col_no,tree in enumerate(row):
                if col_no != 0 and col_no != len(row)-1:
                    if tree > max(trees[row_no,:col_no]) or tree > max(trees[:row_no,col_no]) \
                        or tree > max(trees[row_no,col_no+1:]) or tree > max(trees[row_no+1:,col_no]):
                            visible += 1
                        
    print(visible)
            
    # star 2
    max_scenic_score = 0
    for row_no,row in enumerate(trees):
        if row_no != 0 and row_no != len(trees)-1:
            for col_no,tree in enumerate(row):
                if col_no != 0 and col_no != len(row)-1:
                    for index1,next_tree in enumerate(np.flip(trees[row_no,:col_no])):
                        if next_tree >= tree:
                            scenic1 = index1+1
                            break
                        else:
                            scenic1 = col_no
                    for index1,next_tree in enumerate(trees[row_no,col_no+1:]):
                        if next_tree >= tree:
                            scenic2 = index1+1
                            break
                        else:
                            scenic2 = len(trees)-col_no-1
                    for index1,next_tree in enumerate(np.flip(trees[:row_no,col_no])):
                        if next_tree >= tree:
                            scenic3 = index1+1
                            break
                        else:
                            scenic3 = row_no
                    for index1,next_tree in enumerate(trees[row_no+1:,col_no]):
                        if next_tree >= tree:
                            scenic4 = index1+1
                            break
                        else:
                            scenic4 = len(trees)-row_no-1                    
                            
                    scenic_score = scenic1*scenic2*scenic3*scenic4
                    if scenic_score > max_scenic_score:
                        max_scenic_score = scenic_score
                        
    print(max_scenic_score)
    
main()
