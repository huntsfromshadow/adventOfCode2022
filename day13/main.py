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


def list_compare(left_v, right_v) -> Optional[bool]:
    idx = 0

    print(f"\t- Compare {left_v} vs {right_v}")    

    while True:
        print(idx, len(left_v), len(right_v))
        if idx >= len(left_v) and idx >= len(right_v):
            # We've walked through both lists return procesing to above
            return None
            
        elif idx >= len(left_v) and idx < len(right_v):
            # Left has no elements, right does
            raise Exception("idx >= len(left_v) and idx < len(right_v)")
            
        elif idx < len(left_v) and idx >= len(right_v):
            # Right has no elements, left does
            raise Exception("idx < len(left_v) and idx >= len(right_v)")
            
        else:
            # both have elements available
            le = left_v[idx]
            re = right_v[idx]
            print(f"\t\tComparing {le} vs {re}")            
            if le < re:
                print("1")
                # If left is smaller then right order
                print("- Left side is smaller, so inputs are in the right order")
                return True
            elif re > le:
                print("2")
                # If right is bigger then wrong order
                print("- Right side is smaller, so inputs are in the right order")
                return False
            else:
                print("3")
                idx = idx + 1    


"""


== Pair 2 ==
- Compare [[1],[2,3,4]] vs [[1],4]
  - Compare [1] vs [1]
    - Compare 1 vs 1
  - Compare [2,3,4] vs 4
    - Mixed types; convert right to [4] and retry comparison
    - Compare [2,3,4] vs [4]
      - Compare 2 vs 4
        - Left side is smaller, so inputs are in the right order

== Pair 3 ==
- Compare [9] vs [[8,7,6]]
  - Compare 9 vs [8,7,6]
    - Mixed types; convert left to [9] and retry comparison
    - Compare [9] vs [8,7,6]
      - Compare 9 vs 8
        - Right side is smaller, so inputs are not in the right order

== Pair 4 ==
- Compare [[4,4],4,4] vs [[4,4],4,4,4]
  - Compare [4,4] vs [4,4]
    - Compare 4 vs 4
    - Compare 4 vs 4
  - Compare 4 vs 4
  - Compare 4 vs 4
  - Left side ran out of items, so inputs are in the right order

== Pair 5 ==
- Compare [7,7,7,7] vs [7,7,7]
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Compare 7 vs 7
  - Right side ran out of items, so inputs are not in the right order

== Pair 6 ==
- Compare [] vs [3]
  - Left side ran out of items, so inputs are in the right order

== Pair 7 ==
- Compare [[[]]] vs [[]]
  - Compare [[]] vs []
    - Right side ran out of items, so inputs are not in the right order

== Pair 8 ==
- Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
  - Compare 1 vs 1
  - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
    - Compare 2 vs 2
    - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
      - Compare 3 vs 3
      - Compare [4,[5,6,7]] vs [4,[5,6,0]]
        - Compare 4 vs 4
        - Compare [5,6,7] vs [5,6,0]
          - Compare 5 vs 5
          - Compare 6 vs 6
          - Compare 7 vs 0
            - Right side is smaller, so inputs are not in the right order
"""



#######################################

f = open("short_input.txt")
iput = f.read().splitlines()

pairs = []
wrk = []
for i in iput:

    if i != "":
        wrk.append(json.loads(i))

    if len(wrk) == 2:
        pairs.append(wrk)
        wrk = []


idx = 2
p = pairs[idx]
print(f"== Pair {idx} ==")
d = run_compare(p)

print(f"\n\nResult: {d}")

#for idx, p in enumerate(pairs):
#    print(f"== Pair {idx} ==")
#    run_compare(p)
#    quit()

