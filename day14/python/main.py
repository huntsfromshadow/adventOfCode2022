from asciimatics.screen import ManagedScreen, Screen
from typing import Tuple, List
from asciimatics.scene import Scene
from asciimatics.effects import Cycle, Stars
from asciimatics.renderers import FigletText
from time import sleep
from enum import Enum

class RES(Enum):
    MOVING = 1
    TARGET = 2
    STOPPED = 3

class Wall:
    def __init__(self, start: Tuple[int, int], end: Tuple[int, int] ):
        self.start = start
        self.end = end

    def report_cells(self) -> List[Tuple[int,int]]:
        points = []
        if self.start[0] == self.end[0]:
            # It's a y walk
            y_pnt = [self.start[1], self.end[1]]
            y_pnt.sort()
        
            for y in range(y_pnt[0],y_pnt[1]+1):
                points.append( (self.start[0], y))
        else:
            # It's an x walk
            x_pnt = [self.start[0], self.end[0]]
            x_pnt.sort()
        
            for x in range(x_pnt[0],x_pnt[1]+1):
                # Adjusting to bring X into the screen
                points.append( (x, self.start[1]))
        return points

    def draw(self, screen, xadj):
        points = self.report_cells()
        for p in points:
            screen.print_at("#", p[0] - xadj, p[1])

    def __repr__(self) -> str:
        return "(" + str(self.start[0]) + "," + str(self.start[1]) + " -> " + str(self.end[0]) + "," + str(self.end[1]) + ")"

class Sand:
    def __init__(self, pos: Tuple[int, int]):
        self.pos = pos

    def move_down(self):
        self.pos = (self.pos[0], self.pos[1] + 1)

    def move_left_down(self):
        self.pos = (self.pos[0] - 1, self.pos[1] + 1)

    def move_right_down(self):
        self.pos = (self.pos[0] + 1, self.pos[1] + 1)

    
    def step(self, wall_parts, sand_parts, target) -> RES:
        # Setup Cords
        down = (self.pos[0], self.pos[1]+1)
        left_down = (self.pos[0]-1, self.pos[1]+1)
        right_down = (self.pos[0]+1, self.pos[1]+1)

        down_blocked = down in wall_parts or down in sand_parts
        
        # Move rules
        #  Down
        #  Down Left
        #  Down Right
        if down_blocked == False:
            # we can move down
            self.move_down()

            if self.pos[1] == target[1]+1:
                return RES.TARGET
            else: 
                return RES.MOVING

        else:
            ld_blocked = left_down in wall_parts or left_down in sand_parts
            rd_blocked = right_down in wall_parts or right_down in sand_parts    
            
            if ld_blocked == False:
                # move left down
                self.move_left_down()
                return RES.MOVING
            else:
                if rd_blocked == False:
                    # move right down
                    self.move_right_down()
                    return RES.MOVING
                else:
                    # All blocked
                    return RES.STOPPED
                    
    def draw(self, screen, xadj):
        screen.print_at("*", self.pos[0] - xadj, self.pos[1])


class Cave:
    walls = []
    wall_parts = []

    x_adjustment = 0
    
    sands = []
    sand_parts = []    

    sand_moving = False
    sand_source = (500, 0)

    target = None

    sim_finished = False

    txt_shift = 0


    def add_wall(self, wall: Wall):
        self.walls.append(wall)

    def get_left_base(self):
        return self.get_bases()[0]

    def get_right_base(self):
        return self.get_bases()[1]

    def get_top_base(self):
        return self.get_bases()[2]

    def get_bottom_base(self):
        return self.get_bases()[3]

    def get_bases(self) -> Tuple[int, int, int, int]:
        
        x_cords = []
        y_cords = []

        for w in self.walls:
            x_cords.append(w.start[0])
            x_cords.append(w.end[0])
            y_cords.append(w.start[1])
            y_cords.append(w.end[1])

        return (min(x_cords), max(x_cords), min(y_cords), max(y_cords))

    def calc_x_adjustment(self):
        self.x_adjustment = ((self.get_left_base() - 5))

    def prep_simulation(self):
        self.calc_x_adjustment()
        for w in self.walls:
            self.wall_parts = self.wall_parts + w.report_cells()

        (l, _, _, b) = self.get_bases()
        l = l - 1
        b = b + 1
        self.target = (l, b)
    
    def draw(self, screen):
        # Draw empty spaces first
        (_, r, _, b) = self.get_bases()
        for x in range(0, r - self.x_adjustment + 1):
            for y in range(0, b + 1):
                screen.print_at(".", x, y)
                                
        for w in self.walls:
            w.draw(screen, self.x_adjustment)

        # Sand Source
        screen.print_at("+", self.sand_source[0] - self.x_adjustment, self.sand_source[1] )

        # Sand Here
        for s in self.sands:
            s.draw(screen, self.x_adjustment)

        screen.print_at(len(self.sands), 100, 0)
        
        if len(self.sands) > 0:
            s = self.sands[-1]
            screen.print_at(self.target, 80, 9 + self.txt_shift)
            screen.print_at(s.pos, 80, 10 + self.txt_shift)

        if self.sim_finished:
            screen.print_at("Sim Finished", 100, 1)

    def step(self):
        if self.sim_finished == True:
            return

        if self.sand_moving == False:
            # Generate Sand
            snd = Sand( (self.sand_source[0], self.sand_source[1] ))
            self.sand_moving = True
            self.sands.append(snd)
        
        # Simulate Sand mvmt
        # Cur sand is always last
        s = self.sands[-1]
        res = s.step(self.wall_parts, self.sand_parts, self.target)
        if res == RES.MOVING:
            # We are just moving
            pass
        elif res == RES.STOPPED:
            # We are stopped            
            self.sand_parts.append(s.pos)
            self.sand_moving = False
        else:
            # Done
            self.sim_finished = True


f = open("input.txt")
lines = f.read().splitlines()

cave = Cave()

for l in lines:
    cords = l.split("->")
    pnts = []

    for c in cords:
        p = c.split(",")
        p = (int(p[0]), int(p[1]))
        pnts.append(p)

    for idx in range(len(pnts)):
        if( idx == 0):
            continue
        else:
            cave.add_wall( Wall(pnts[idx - 1], pnts[idx]))

cave.prep_simulation()

@ManagedScreen
def demo(screen=None):
    while True:
        #screen.print_at("#", p[0], p[1] )

        ev = screen.get_key()
        if ev in (ord('Q'), ord('q')):
            return
        
        if ev == Screen.KEY_DOWN:
            screen.scroll(1)
            cave.txt_shift = cave.txt_shift + 1

        if ev == Screen.KEY_UP:
            screen.scroll(-1)
            cave.txt_shift = cave.txt_shift - 1

        #if ev == 32:
        cave.step()
   
        cave.draw(screen)
        screen.refresh()

demo()





