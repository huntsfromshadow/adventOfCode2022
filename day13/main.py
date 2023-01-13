import copy
import json
from typing import Optional

def run_compare(pair) -> bool:
    left = pair[0]
    right = pair[1]

    print(f"- Compare {left} vs {right}")

    idx = 0 # Start Index
    while True:
        left_v = left[idx]
        right_v = right[idx]

        # Are both Int?
        if type(left_v) == int and type(right_v) == int:
            print(f"\t- Compare {left_v} vs {right_v}")
            if left_v < right_v:
                print(f"\t\t- Left side is smaller, so inputs are in the right order")
                return True
            elif left_v > right_v:
                print(f"\t\t- Right side is smaller, so inputs are not in the right orde")
                return False
            else:
                # Move Index ahead and let loop operate
                idx = idx + 1        
        elif type(left_v) == list and type(right_v) == list:
            # Are both lists?
            v = list_compare(left_v, right_v)
            if v == None:
                idx = idx + 1
            else:
                return v
        else:
            # One Side is list the other isn't
            (lconv, rconv) = convert(left_v, right_v)

            # Both sides are now lists
            v = list_compare(lconv, rconv)
            if v == None:
                idx = idx + 1
            else:
                return v

def convert(left_v, right_v):
    if type(left_v) == int:
        c = [left_v]
        print(f"- Mixed Types; convert left to {c} and retry comparison")
        return (c, right_v)
    else:
        c = [right_v]
        print(f"- Mixed Types; convert right to {c} and retry comparison")
        return (left_v, c)

def r_comp(left, right) -> Optional[bool]:

    print(f"Rcomp starting {left}, {right}")

    while True:
        print(f"Loop Start left: {left}")
        print(f"Loop start right: {right}")
        # check for empty conditions
        if type(left) == int and type(right) == int:
            print(f"Compare {left} vs {right}")
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
                    print("Move on")
                    continue    # Continue on
                else:
                    return res



########################################






f = open("input.txt")
iput = f.read().splitlines()
pairs = []
wrk = []
for i in iput:

    if i != "":
        wrk.append(json.loads(i))

    if len(wrk) == 2:
        pairs.append(wrk)
        wrk = []

idx_total = 0
for idx, p in enumerate(pairs):
    print(f"== Pair {idx} ==")
    r = r_comp( copy.deepcopy(p[0]), copy.deepcopy(p[1]))
    print(f"\tResult: {r}")

    if r:
        idx_total = idx_total + idx + 1

print(f"\nFinal Result: {idx_total}")


