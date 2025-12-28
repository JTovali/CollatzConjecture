import sys
import os

# compute the collatz sequence length
def collatz_length(n):
    length = 0
    while n != 1:
        length += 1
        if n % 2 == 0:
            n //= 2
        else:
           n = 3 * n + 1
    return length
        

# find the 10 smallest integers with the longest collazt sequence length
def find_the_top_10_collatz_integers(start, end):
    # only keep at most 10 pairs
    top = []

    for n in range(start, end + 1):
        length = collatz_length(n)

        # if length already exist then keep the one with the smallest integer
        same_idx = None
        for i, (num_i, len_i) in enumerate(top): # (integer, length)
            if len_i == length:  # have encountered the same length
                same_idx = i
                break
        if same_idx is not None: # compare integer and keep the smallest int
            if n < top[same_idx][0]:
                top[same_idx][0] = n   # n is smaller than the integer stored
            continue  

        # if new length and top is not full, then add
        if len(top) < 10:
            top.append([n, length])
        else:
            # new length but top is full, then replace with the worst - good = big length, small int
            worst_idx = 0
            for i in range(1, len(top)):
                ni, li = top[i]  # integeri, lengthi
                nw, lw = top[worst_idx]  # integer, length at worst position
                # worst is smaller length or same length but larger integer
                if li < lw or (li == lw and ni > nw):
                    worst_idx = i

            worst_n, worst_l = top[worst_idx]

            # replace worst if the the current pair is better
            if (length > worst_l) or (length == worst_l and n < worst_n):
                top[worst_idx] = [n, length]

    # sort in length descending and integer ascending
    top.sort(key=lambda x: (-x[1], x[0]))
    return top

def clear_screen():
    os.system("clear")
    sys.stdout.flush()

def main():
    if len(sys.argv) != 3:
        print("Usage: python3 program_name.py start end")
        print(" Example: python3 collatz.py 50 100")
        sys.exit(1)
        
    try: 
        # convert to integer and safely check
        x = int(sys.argv[1]) 
        y = int(sys.argv[2])
    except ValueError:
        print("Error while converting to integer. Please provide 2 integers for x and y.")
        sys.exit(1)
    
    # find minimum and maximum of the 2 integers to find range    
    start = min(x,y) 
    end = max(x,y)
    
    # print("Start: " + str(start) + " End: " + str(end))  # testing

    top_10 = find_the_top_10_collatz_integers(start, end)
        
    clear_screen()
    print("Sorted based on sequence length")
    for num, length in top_10:
        print(f"                {num}               {length}")

    print()
    print("Sorted based on integer size")
    temp_top_10 = top_10.copy()
    temp_top_10.sort(key=lambda x: (-x[0]))
    for num, length in temp_top_10:
        print(f"                {num}               {length}")
    
if __name__ == "__main__":
    main()    
