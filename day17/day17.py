from typing import List

blocks = [
    [["@","@","@","@"]], # Horizontal
    [[".", "@", "."], ["@", "@", "@"], [".", "@", "."]], # Plus
    [[".", ".", "@"], [".", ".", "@"], ["@", "@", "@"]], # Corner
    [["@"],["@"],["@"],["@"]], # Vertical
    [["@", "@"], ["@", "@"]] ] # Square
blocks_idx = 0

instr = []
instr_idx = 0
file = open("short-input.txt")
txt = file.read()
txt = txt.replace("\n", "")
for i in txt:
    instr.append(i)

#######################

# Cords are Y,X grid with 0,0 at bottom left
class Grid:
    data = []

    def __init__(self):
        # Set up the first 3 rows
        self.new_empty_row()
        self.new_empty_row()
        self.new_empty_row()

    def get(self, row: int, col: int) -> str:
        return self.data[row][col]

    def put(self, row: int, col: int, val: str):
        if row >= len(self.data):
            raise Exception("Row out of bounds")
        else:
            self.data[row][col] = val

    def new_empty_row(self):
        self.data.append( ["."] * 7 )

    def show_grid(self):
        nc = self.data[::-1]
        for row in nc:
            for col in row:
                print(col, end="")
            print("")

    def get_last_row(self) -> int:
        return len(self.data) - 1


def place_block(grid: Grid, block: List[List]):
    n_blk = block[::-1]

    for row in n_blk:
        nrow = ['.', '.'] + row + (['.'] * (7 - 2 - len(row)))
        grid.data.append(nrow)        
    

def can_move_right(grid: Grid, row_idx: int, col_idx: int) -> bool:
    # First can we actually move right from the col_idx?
    if col_idx == 6:
        return False
    else:
        v = grid.get(row_idx, col_idx + 1)
        if v == "." or v == "@": 
            return True
        else:
            return False


def can_move_down(grid: Grid, row_idx: int, col_idx: int) -> bool:
    if row_idx == 0:
        return False
    else:
        v = grid.get(row_idx - 1, col_idx)
        if v == "." or v == "@":
            return True
        else:
            return False


def shift_block_right(grid: Grid):
    for row_idx in range(0, len(grid.data)):
        for col_idx in range(6, -1, -1):
            v = grid.get(row_idx, col_idx)
            if v == "@":
                grid.put(row_idx, col_idx, ".")
                grid.put(row_idx, col_idx+1, "@")


def shift_block_down(grid: Grid):
    pass

def air_blast(grid: Grid, ins: str) -> bool:
    # Walk down the columns
    if ins == ">":
        for col_idx in range(6, -1, -1):
            for row_idx in range(len(grid.data)-1, -1, -1):                
                v = grid.get(row_idx, col_idx)
                print(row_idx, "--", col_idx, " --> ", v)

                if v == "@":
                    # Okay it's a moving block so we need to check to the right
                    if can_move_right(grid, row_idx, col_idx) == False:
                        # Nope return
                        return False
         
        # If we made it here we can do the move
        shift_block_right(grid)
        return True

    else:
        raise Exception("Move Left")


def move_down(grid: Grid):
    handle_move_down = False
    for row_idx in range(0, len(grid.data)):
        if "@" in grid[row_idx]:
            # Unless their is a @ we just move on
            for col_idx in range(0, 7):
                v = grid.get(row_idx, col_idx)
                if v == "@":
                    # Okay it's a brick so can it move down
                    if can_move_down(row_idx, col_idx) == False:
                        handle_move_down = True
    
                




###################


# Start by putting in the bottom 3 rows
grid = Grid()

place_block(grid, blocks[blocks_idx])
blocks_idx = blocks_idx + 1
grid.show_grid()

air_blast(grid, instr[instr_idx])
instr_idx = instr_idx + 1
grid.show_grid()

move_down(grid)
grid.show_grid()