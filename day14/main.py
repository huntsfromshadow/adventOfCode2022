from asciimatics.screen import ManagedScreen
from typing import Tuple
from asciimatics.scene import Scene
from asciimatics.effects import Cycle, Stars
from asciimatics.renderers import FigletText
from time import sleep

class Wall:
    def __init__(self, start: Tuple[int, int], end: Tuple[int, int] ):
        self.start = start
        self.end = end

    def draw(self, screen, xadj):
        #print(self.start[0]-xadj, self.start[1])
        screen.print_at("#", self.start[0] - xadj, self.start[1])

    def __repr__(self) -> str:
        return "(" + str(self.start[0]) + "," + str(self.start[1]) + " -> " + str(self.end[0]) + "," + str(self.end[1]) + ")"

class Cave:
    walls = []
    x_adjustment = 0

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
        left_base = 0
        right_base = 10000000
        top_base = 0
        bottom_base = 1000000

        for w in self.walls:
            if w.start[0] >= left_base:
                left_base = w.start[0]

            if w.start[1] >= top_base:
                top_base = w.start[1]

            if w.end[0] <= right_base:
                right_base = w.start[0]

            if w.start[1] <= bottom_base:
                bottom_base = w.start[1]

        return (left_base, right_base, top_base, bottom_base)

    def calc_x_adjustment(self):
        print(self.get_left_base())
        self.x_adjustment = ((self.get_left_base() - 5) - 400)

    def draw(self, screen):
        for w in self.walls:
            w.draw(screen, self.x_adjustment)


f = open("short_input.txt")
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

cave.calc_x_adjustment()

@ManagedScreen
def demo(screen=None):



    while True:
        #screen.print_at("#", p[0], p[1] )

        cave.draw(screen)

        ev = screen.get_key()
        if ev in (ord('Q'), ord('q')):
            return
        screen.refresh()

demo()
