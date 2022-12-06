
file1 = open('input.txt', 'r')
Lines = file1.readlines()

# a = rock
# b = paper
# c = Scissors

# x = rock = 1
# y = paper = 2
# z = Scissors = 3

def getRoundResult(opponent, me):

    # draw
    if (opponent == me):
        return 3 + getHandValue(me)
    # winning against rock
    if ((opponent == "rock") and (me == "paper")):
        return 6 + getHandValue(me)
    # loosing against rock  
    if ((opponent == "rock") and (me == "scissors")):
        return getHandValue(me)
    # winning against paper
    if ((opponent == "paper") and (me == "scissors")):
        return 6 + getHandValue(me)
    # loosing against paper  
    if ((opponent == "paper") and (me == "rock")):
        return getHandValue(me)
    # winning against scissors
    if ((opponent == "scissors") and (me == "rock")):
        return 6 + getHandValue(me)
    # loosing against scissors  
    if ((opponent == "scissors") and (me == "paper")):
        return getHandValue(me)


def getProperHandName(hand):
    match hand:
        case "A":
            return "rock"
        case "X":
            return "rock"
        case "B":
            return "paper"
        case "Y":
            return "paper"
        case "C":
            return "scissors"
        case "Z":
            return "scissors"

def getHandValue(hand):
    match hand:
        case "rock":
            return 1
        case "paper":
            return 2
        case "scissors":
            return 3

sum = 0
for line in Lines:
    opponent, me = line.strip().split(' ')
    sum += getRoundResult(getProperHandName(opponent), getProperHandName(me))
print(f"Star 1: Total sum is {sum}")


def getRequiredHand(opponent, desiredOutcome):
    # loose
    if desiredOutcome == "X":
        if( opponent == "A"):
            return "scissors"
        if(opponent == "B"):
            return "rock"
        if( opponent == "C"):
            return "paper"
    # draw
    if desiredOutcome == "Y":
        return getProperHandName(opponent)
    # win
    if desiredOutcome == "Z":
        if( opponent == "A"):
            return "paper"
        if(opponent == "B"):
            return "scissors"
        if( opponent == "C"):
            return "rock"

sum = 0
for line in Lines:
    opponent, me = line.strip().split(' ')
    sum += getRoundResult(getProperHandName(opponent), getRequiredHand(opponent, me))
print(f"Star 2: Total sum is {sum}")