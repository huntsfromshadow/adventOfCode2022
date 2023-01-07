from __future__ import annotations
import argparse
from typing import Tuple
from enum import Enum

parser = argparse.ArgumentParser(description='File to process')
parser.add_argument('filename')
args = parser.parse_args()

file = open(args.filename, "r")
#file = open("short_input.txt")
lines = file.read().splitlines()

total_moves = 0

##############################

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

def tail_follow() -> TAIL_MOVE :
    if(head_loc.is_location_overlap(tail_loc) \
        or head_loc.is_location_adjacent(tail_loc)):
        return TAIL_MOVE.NONE
    
    if(head_loc.is_location_two_away(tail_loc)):
        if(head_loc.is_location_two_up(tail_loc)):
            tail_loc.shift_loc(0, 1)
        elif(head_loc.is_location_two_left(tail_loc)):
            tail_loc.shift_loc(-1, 0)
        elif(head_loc.is_location_two_down(tail_loc)):
            tail_loc.shift_loc(0, -1)
        else: # It's right
            tail_loc.shift_loc(1, 0)

        return TAIL_MOVE.CARDINAL
    else:
        # diagonal        
        if( head_loc.y > tail_loc.y ): #head above tail
            if( head_loc.x > tail_loc.x) : # head to right of tail
                tail_loc.shift_loc(1,1)
            else: # head left of tail
                tail_loc.shift_loc(-1,1)
            return TAIL_MOVE.DIAGONAL
        else: # head is below tail
            if( head_loc.x > tail_loc.x) : # head to right of tail
                tail_loc.shift_loc(1,-1)
            else: # head left of tail
                tail_loc.shift_loc(-1,-1)
            return TAIL_MOVE.DIAGONAL
        

def move_head(direc: str, amt: int):
    
    for _ in range(0, amt):    

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

        head_str_sloc = head_loc.__repr__()
        tail_str_sloc = tail_loc.__repr__()

        head_loc.shift_loc_by_tuple(d)
        print( f"Head Move {direc} St {head_str_sloc} -> End {head_loc}" )

        res = tail_follow()
        if(res == TAIL_MOVE.NONE):            
            print( f"Tail No Move Needed Loc {tail_str_sloc}")
        elif(res == TAIL_MOVE.DIAGONAL):
            print( f"Tail Move Diag St {tail_str_sloc} -> {tail_loc}")
        else: 
            print( f"Tail Move Card St {tail_str_sloc} -> {tail_loc}")

        if(res != TAIL_MOVE.NONE):
            if (tail_loc.x, tail_loc.y) not in tail_visits:
                tail_visits.append( (tail_loc.x, tail_loc.y) )

        #print_grid(head_loc, tail_loc)
    
def print_grid(head: Location, tail: Location, only_visit=False):
    width = 6
    height = 6

    cells = {}

    for h in range(height-1, -1, -1):
        for w in range(width):
            if (w,h) in tail_visits:
                cells[(w,h)] = "#"
            else:
                cells[(w,h)] = "."

    if not only_visit:
        cells[(tail.x, tail.y)] = "T"    
        cells[(head.x, head.y)] = "H"



    for h in range(height-1, -1, -1):
        out = ""
        for w in range(width):
            out = out + cells[(w,h)] + " "
        print(out)
    

    print("\n=======================\n")

############################

head_loc = Location(0,0)
tail_loc = Location(0,0)
total_moves = 0
tail_visits = []

print("Start Grid")
print_grid(head_loc, tail_loc)

for l in lines:
    d = l.split(" ")
    move_head(d[0], int(d[1]))
    total_moves = total_moves + int(d[1])

#print(tail_visits)
print(f"Total Moves {total_moves}")
print(f"Visit with start {len(tail_visits)+1}")

# print_grid(head_loc, tail_loc, True)
    
