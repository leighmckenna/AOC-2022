import numpy as np

def sum_elf_calories(elf_array):
    sum_array = []

    for elf in elf_array:
        calorie_count = 0
        for calorie in elf:
            calorie_count += calorie
        sum_array.append(calorie_count)

    return sum_array

def parse_data(Lines):
    elf_index = 0
    calorie_index = 0

    elf_array = []
    calorie_array = []

    for line in Lines:
        if calorie_index == 0:
            calorie_array = []
        if line == "\n":
            elf_array.append(calorie_array)
            elf_index += 1
            calorie_index = 0
        else:
            calorie_array.append(int(line))
            calorie_index += 1

    return elf_array

def main():    
    data = open('rawdata.txt', 'r')
    Lines = data.readlines()
    

    elf_array = parse_data(Lines)
    sum_array = sum_elf_calories(elf_array)

    print("answer A: ")
    print(np.nanmax(sum_array))

    print("answer B: ")




main()