
file1 = open('input.txt', 'r')
Lines = file1.readlines()

overlap_cnt = 0
for line in Lines:
  one, two = line.strip().split(",")
  elve_one = one.split("-")
  elve_one = list(map(int, elve_one))
  elve_two = two.split("-")
  elve_two = list(map(int, elve_two))  

  if (((elve_one[0] >= elve_two[0]) and (elve_one[1] <= elve_two[1])) or
       ((elve_two[0] >= elve_one[0]) and (elve_two[1] <= elve_one[1]))):
       overlap_cnt += 1
 
print(f"Star 1: {overlap_cnt} pairs where one is full in the other")


overlap_cnt = 0
for line in Lines:
  one, two = line.strip().split(",")
  elve_one = one.split("-")
  elve_one = list(map(int, elve_one))
  elve_two = two.split("-")
  elve_two = list(map(int, elve_two))  

  if ( ((elve_one[0] >= elve_two[0]) and (elve_one[0] <= elve_two[1])) or
       ((elve_one[1] >= elve_two[0]) and (elve_one[1] <= elve_two[1])) or
       ((elve_two[0] >= elve_one[0]) and (elve_two[0] <= elve_one[1])) or
       ((elve_two[1] >= elve_one[0]) and (elve_two[1] <= elve_one[1]))
       ):
       overlap_cnt += 1
 
print(f"Star 2: {overlap_cnt} pairs where one is full in the other")
