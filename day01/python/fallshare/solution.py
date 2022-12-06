file1 = open('input.txt', 'r')
Lines = file1.readlines()

elves = []

elve = []
for line in Lines:
    if line == "\n":
        elves.append(elve)
        elve = []
    else:
         elve.append(int(line))
# don't forget to add the last elve
elves.append(elve)


maxCal = 0
for elve in elves:
    cal = sum(elve)
    if cal > maxCal:
        maxCal = cal

print(f"Star 1: The biggest amount of calories is: {maxCal}")

calories = []
for elve in elves:
    calories.append(sum(elve))
calories.sort()
highestCalories = calories[-3:]

print(f"Star 2: The sum of the three highest calorie package is: {sum(highestCalories)}")
