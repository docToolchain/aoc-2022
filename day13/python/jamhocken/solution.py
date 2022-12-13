def process_input(file_contents):
    lines_stripped = [line.strip() for line in file_contents]

    signals = list()
    for line in lines_stripped:
        if line != '':
            signal = process_line(line)
            signals.append(signal)
            
    return signals

def process_line(line):
    signal = list()
    n = 1
    while line[n] != ']':
        if line[n].isdigit():
            m = n
            while line[n] not in [',',']']:
                n = n+1
            signal.append(int(line[m:n]))
            if line[n] == ",":
                n = n+1
        else:
            m = n
            n = n+1
            count = 0
            while count != 1:
                if line[n] == "[":
                    count -= 1
                elif line[n] == "]":
                    count += 1
                    
                n = n + 1
            signal.append(process_line(line[m:n]))
            if line[n] == ',':
                n = n+1
    
    return signal

def compare_signals(signal1,signal2):
    if not signal1:
        if not signal2:
            return
        else:
            return True
        
    for index,item in enumerate(signal1):
        if type(item) is int:
            if index == len(signal2):
                if len(signal1)>len(signal2):
                    return False
            
            if type(signal2[index]) is int:
                if item > signal2[index]:
                    return False
                elif item < signal2[index]:
                    return True
            else:
                status = compare_signals([item],signal2[index])
                if status in [True,False]:
                    return status
            
            if index == len(signal1)-1:
                if len(signal1)<len(signal2):
                    return True
        else:
            if index == len(signal2):
                if len(signal1)>len(signal2):
                    return False
    
            if type(signal2[index]) is int:
                status = compare_signals(item,[signal2[index]])
                if status in [True,False]:
                    return status  
            else:
                status = compare_signals(item,signal2[index])
                if status in [True,False]:
                    return status  
    
            if index == len(signal1)-1:
                if len(signal1)<len(signal2):
                    return True    
    
    return

def bubble_sort(array):
    n = len(array)

    for i in range(n):
        already_sorted = True
        
        for j in range(n - i - 1):
            if not compare_signals(array[j], array[j + 1]):
                array[j], array[j + 1] = array[j + 1], array[j]
                already_sorted = False

        if already_sorted:

            break

    return array

def main():
    with open("input.txt",'r') as input_file:
        input_lines = input_file.readlines()

    #star 1
    signals = process_input(input_lines)
    
    i = 0
    sum_indices = 0
    while i < len(signals):
        if compare_signals(signals[i],signals[i+1]):
            sum_indices += i//2 +1
        i = i+2

    print(sum_indices)

    #star 2
    signals.extend([[[2]],[[6]]])

    new_signals = bubble_sort(signals)
    
    decoder = 1
    i = 1
    for signal in new_signals:
        if signal in [[[2]],[[6]]]:
             decoder *= i
        i = i +1
        
    print(decoder)
             

main()
