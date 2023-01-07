from __future__ import annotations
import argparse
from typing import Tuple, List, Dict
from enum import Enum

NUM_KNOTS = 9

parser = argparse.ArgumentParser(description='File to process')
parser.add_argument('filename')
args = parser.parse_args()

file = open(args.filename, "r")
#file = open("short_input.txt")
lines = file.read().splitlines()

total_moves = 0

##############################

class Rope:
    head_loc = None
    knots: List[Location] = []
    tail_visits: Dict[int, List[tuple]] = {}

    def __init__(self):
        self.head_loc = Location(0,0)
        for idx in range(NUM_KNOTS):
            self.knots.append(Location(0,0))
            self.tail_visits[idx] = []
    
    def move_head(self, direc: str, amt: int):
        for cnt in range(0, amt):    
            d = (0,0)
            if direc == "U":
                d = (0,1)            
            elif direc == "L":
                d = (-1,0)
            elif direc == "D":
                d = (0,-1)
            elif direc == "R":
                d = (1,0)
            else:
                print("No idea what direc is:", direc)

            head_str_sloc = self.head_loc.__repr__()
            knots_orign_pos = list(map(lambda x: x.__repr__(), self.knots))

            self.head_loc.shift_loc_by_tuple(d)
            print( f"Head Move {direc} - {cnt} of {amt} -- St {head_str_sloc} -> End {self.head_loc}" )

            for idx in range(NUM_KNOTS):
                res = self.knot_follow(idx)
    
                if(res == TAIL_MOVE.NONE):            
                    print( f"Tail {idx} - No Move Needed Loc {knots_orign_pos[idx]}")
                elif(res == TAIL_MOVE.DIAGONAL):
                    print( f"Tail {idx} - Move Diag St {knots_orign_pos[idx]} -> {self.knots[idx]}")
                else: 
                    print( f"Tail {idx} - Move Card St {knots_orign_pos[idx]} -> {self.knots[idx]}")

                if(res != TAIL_MOVE.NONE):
                    d = self.tail_visits[idx]

                    if (self.knots[idx].x, self.knots[idx].y) not in d:
                        d.append( (self.knots[idx].x, self.knots[idx].y) )

            #print_grid(self)


    def knot_follow(self, idx: int) -> TAIL_MOVE :
        leader = None
        follower = self.knots[idx]

        if idx == 0:
            leader = self.head_loc
        else:
            leader = self.knots[idx-1]

        if(leader.is_location_overlap(follower) \
            or leader.is_location_adjacent(follower)):
            return TAIL_MOVE.NONE
    
        if(leader.is_location_two_away(follower)):
            if(leader.is_location_two_up(follower)):
                follower.shift_loc(0, 1)
            elif(leader.is_location_two_left(follower)):
                follower.shift_loc(-1, 0)
            elif(leader.is_location_two_down(follower)):
                follower.shift_loc(0, -1)
            else: # It's right
                follower.shift_loc(1, 0)

            return TAIL_MOVE.CARDINAL
        else:
            # diagonal        
            if( leader.y > follower.y ): #head above tail
                if( leader.x > follower.x) : # head to right of tail
                    follower.shift_loc(1,1)
                else: # head left of tail
                    follower.shift_loc(-1,1)
                return TAIL_MOVE.DIAGONAL
            else: # head is below tail
                if( leader.x > follower.x) : # head to right of tail
                    follower.shift_loc(1,-1)
                else: # head left of tail
                    follower.shift_loc(-1,-1)
                return TAIL_MOVE.DIAGONAL

class TAIL_MOVE(Enum):
    NONE = 1
    CARDINAL = 2
    DIAGONAL = 3


class Location:

    x = 0
    y = 0

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return(f"({self.x}, {self.y})")

    def shift_loc(self, x: int, y: int):
        self.x = self.x + x
        self.y = self.y + y

    def shift_loc_by_tuple(self, v: Tuple[int, int] ):
        self.shift_loc( v[0], v[1])

    def is_location_overlap(self, loc: Location) -> bool:
        return (self.x == loc.x) and (self.y == loc.y)

    def is_location_adjacent(self, loc: Location) -> bool:
        # Up
        u = (loc.x == self.x) and (loc.y == (self.y + 1))
        # Up-Left
        ul = (loc.x == (self.x - 1)) and (loc.y == (self.y + 1))
        # Left
        l = (loc.x == (self.x - 1)) and (loc.y == self.y)        
        # Down-Left
        dl = (loc.x == (self.x-1)) and (loc.y == (self.y - 1))
        # Down
        d = (loc.x == self.x) and (loc.y == (self.y - 1))
        # Down-Right
        dr = (loc.x == (self.x + 1)) and (loc.y == (self.y - 1))    
        # Right
        r = (loc.x == (self.x + 1)) and (loc.y == self.y)
        # Up-Right
        ur = (loc.x == (self.x + 1)) and (loc.y == (self.y + 1))
        
        return u or ul or l or dl or d or dr or r or ur

    def is_location_two_up(self, loc: Location) -> bool:
        return loc.x == self.x and (loc.y == (self.y - 2))

    def is_location_two_left(self, loc: Location) -> bool:
        return (loc.x == (self.x + 2)) and (loc.y == self.y)

    def is_location_two_down(self, loc: Location) -> bool:
        return loc.x == self.x and (loc.y == (self.y + 2))

    def is_location_two_right(self, loc: Location) -> bool:
        return (loc.x == (self.x - 2)) and (loc.y == self.y)

    def is_location_two_away(self, loc: Location) -> bool:
        return self.is_location_two_up(loc) or \
            self.is_location_two_left(loc) or \
            self.is_location_two_right(loc) or \
            self.is_location_two_down(loc) 

def print_grid(rope: Rope):
    width = 10
    height = 10

    cells = {}

    for h in range(height-1, -1, -1):
        for w in range(width):
            #if (w,h) in tail_visits:
            #    cells[(w,h)] = "#"
            #else:
            cells[(w,h)] = "."

    #if not only_visit:
        
    for idx in range(NUM_KNOTS-1,-1,-1):
        cells[(rope.knots[idx].x, rope.knots[idx].y)] = str(idx + 1)

    cells[(rope.head_loc.x, rope.head_loc.y)] = "H"



    for h in range(height-1, -1, -1):
        out = ""
        for w in range(width):
            out = out + cells[(w,h)] + " "
        print(out)
    

    print("\n=======================\n")

############################


rope = Rope()


print("Start Grid")
#print_grid(rope)

for l in lines:
    d = l.split(" ")
    rope.move_head(d[0], int(d[1]))
    #total_moves = total_moves + int(d[1])

print(rope.tail_visits)
for k,v in rope.tail_visits.items():
    print(f"Idx: {k} - Visits: {len(v)}")

#print(tail_visits)
#print(f"Total Moves {total_moves}")
#print(f"Visit with start {len(tail_visits)+1}")

# print_grid(head_loc, tail_loc, True)
    
