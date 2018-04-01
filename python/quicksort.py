#!/usr/env python2

def quicksort(numbers):
    #print(numbers)
    if len(numbers)>1:
        left = numbers[0]
        return quicksort(filter(lambda x: x<left, numbers))+ \
               [left]+ \
               quicksort(filter(lambda x: x>left, numbers))
    else:
        return numbers
    
if __name__ == '__main__':
    from random import sample
    
    arr = sample(range(100), 20)
    print(arr)
    print(quicksort(arr))