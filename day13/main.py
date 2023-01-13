import copy
import json
from typing import Optional

def r_comp(left, right) -> Optional[bool]:

    #print(f"Rcomp starting {left}, {right}")

    while True:
        #print(f"Loop Start left: {left}")
        #print(f"Loop start right: {right}")
        # check for empty conditions
        if type(left) == int and type(right) == int:
            #print(f"Compare {left} vs {right}")
            if left == right:
                return None
            elif left < right:
                return True
            else:
                return False

        elif type(left) != list or type(right) != list:
            # One of the sides is not a list while the other is
            if type(left) != list:
                left = [left]
            else:
                right = [right]

            v = r_comp(left, right)

            if v == None:
                return None
            else:
                return v

        else:

            if len(left) == 0 and len(right) == 0:
                # Both 0 move on
                return None

            elif len(left) == 0 and len(right) != 0:
                return True

            elif len(left) != 0 and len(right) == 0:
                return False

            else:
                # Both are Lists
                left_elem = left.pop(0)
                right_elem = right.pop(0)

                res = r_comp(left_elem, right_elem)

                if res == None:
                    #print("Move on")
                    continue    # Continue on
                else:
                    return res

########################################

f = open("input.txt")
iput = f.read().splitlines()
#pairs = []
#wrk = []
data = []
for i in iput:
    if i != "":
        data.append(json.loads(i))

data.append([[2]])
data.append([[6]])

# Old fashioned slow sort
idx = 0
did_swap = False
while True:
    start_idx = idx
    target_idx = idx + 1

    print(f"{start_idx} -> {target_idx}")


    result = r_comp(copy.deepcopy(data[start_idx]), copy.deepcopy(data[target_idx]))

    print(f"\tResult: {result}")

    if result == False:
        did_swap = True
        tmp = data[start_idx]
        data[start_idx] = data[target_idx]
        data[target_idx] = tmp

    idx = idx + 1

    if(idx == len(data)-1):
        print(f"Run -> {data}")
        if did_swap == True:
            idx = 0
            did_swap = False
        else:
            break

divider1 = None
divider2 = None
for idx, l in enumerate(data):
    print(l)

    if l == [[2]]:
        divider1 = idx + 1

    if l == [[6]]:
        divider2 = idx + 1

print(divider1 * divider2)
















