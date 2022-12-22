
def main():
    file = open('input.txt', 'r')

    buffer = file.read()   
    print(f"Star 1: Found start-of-packet marker at: {find_uniqe_sequence(buffer, 4)}")
    print(f"Star 2: Found start-of-packet marker at: {find_uniqe_sequence(buffer, 14)}")

def find_uniqe_sequence(buffer, bucketSize):
    bucket = []
    for i in range(0,bucketSize - 1):
        bucket.insert(0,buffer[i])

    for x in range(bucketSize - 1, len(buffer)):
        bucket.insert(0,buffer[x])
        if len(bucket) == len(set(bucket)):
            return x + 1
        else:
            bucket.pop()

def star_1(buffer):
    bucket = []
    bucket.insert(0,buffer[0])
    bucket.insert(0,buffer[1])
    bucket.insert(0,buffer[2])
    for x in range(3, len(buffer)):
        bucket.insert(0,buffer[x])
        if len(bucket) == len(set(bucket)):
            print(f"Star 1: Found start-of-packet marker at: {x + 1}")
            break
        else:
            bucket.pop()

if __name__ == '__main__':
    main()