from itertools import repeat

INPUT = "short-input.txt"

class JetInstructions:
    next_i = None
    inst = None

    def __init__(self, filename):
        f = open(INPUT)
        raw = f.read()

        self.next_i = 0
        self.inst = []
        raw.replace('\n','')
        for c in raw:
            self.inst.append(c)

    def get_next_blast(self):
        ret = self.inst[self.next_i]
        self.next_i = self.next_i + 1
        if self.next_i == len(self.inst):
            self.next_i = 0
        return ret

class Grid:
    """
    -1 - Stopped
    0 - Empty
    1 - Moving
    """
    rows = []
    jetins = None

    def __init__(self, jetins):
        self.jetins = jetins

    def show_grid(self):
        nr = self.rows[::-1]
        cnt = len(self.rows) - 1
        for r in nr:
            ls = ""
            for c in r:
                if c == 0:
                    ls = ls + " . "
                elif c == 1:
                    ls = ls + " @ "
                else:
                    ls = ls + " # "

            print( f"{cnt} -- |", ls, "|")
            cnt = cnt - 1

        
    def place_block(self, block):
        # First we need to put 3 empty rows above bottom
        self.rows.append( list(repeat(0,7)) )
        self.rows.append( list(repeat(0,7)) )
        self.rows.append( list(repeat(0,7)) )

        # Now we place the bottom of the block and leave the rest in storage for later
        block.reverse()
        for br in block:
            self.rows.append( [0, 0] + br + list(repeat(0, 5-len(br))))
            
    def blast_air(self):
        # get next instr
        next = self.jetins.get_next_blast()

        for y in range(0, len(self.rows)):
            r = self.rows[y]

            if max(r) == 0:
                # Skipping as it's either all 0 or all 0 and -1
                print("Skipping ", y)
                pass
            else:
                # So their is actually a block in here
                if next == ">":
                    self.shift_right(y)                            
                    #self.show_grid()
                else:
                    raise Exception("Not here yet")

    
    def step_down(self):

    
    
    def shift_right(self, y):
        # Grab row
        r = self.rows[y]

        # First see if it will hit wall if so we cant do anything anyway
        if r[len(r) - 1] == 1:
            # So we have a space in the far right field so we can't move anyway
            return
        else:
            if min(r) == 0 and max(r) == 1:
            # Yep just an block row
            # Is their room to move it (will it hit wall)
                wrk = r[:]
                self.rows[y] = [wrk.pop(-1)] + wrk
                return
            else:
                raise Exception("Come back to fix")
        

        

class Blocks:
    def __init__(self):
        self.next_block = 2
    
    def get_next_block(self):
        if self.next_block == 0:
            self.next_block = self.next_block + 1
            return [[1,1,1,1]]
        
        elif self.next_block == 1:
            self.next_block = self.next_block + 1
            return [
                [0, 1, 0],
                [1, 1, 1],
                [0, 1, 0]]
            
        elif self.next_block == 2:
            self.next_block = self.next_block + 1
            return [
                [0, 0, 1],
                [0, 0, 1],
                [1, 1, 1]]
        
        elif self.next_block == 3:
            self.next_block = self.next_block + 1
            return [
                [1],
                [1],
                [1],
                [1]]

        elif self.next_block == 4:
            self.next_block = 0
            return [
                [1, 1],
                [1, 1]]
        
        else:
            print("I have no idea on what block")


##### Main

jetins = JetInstructions(INPUT)
grid = Grid(jetins)
blocks = Blocks()

# Logic
next_block =  blocks.get_next_block()
grid.place_block(next_block)
grid.show_grid()

grid.blast_air()
grid.step_down()



