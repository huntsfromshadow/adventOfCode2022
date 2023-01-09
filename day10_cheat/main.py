from typing import List
import os

FILENAME = "../day10/input.txt"

class Console():
    def __init__(self) -> None:
        self.width: int = 40
        self.height: int = 6
        self.dots: List[str] = [" " * self.width] * self.height
        self.x: int = 0
        self.y: int = 0

    def set_char(self, x, y, sprite):
        self.dots[y] = self.dots[y][:x] + sprite + self.dots[y][x + 1:]

    def draw(self, p: int):
        # called each cycle
        # p is an ON pixel +/- 1.
        # if screen is currently drawing a pixel that matches P +/- 1, then
        # that pixel is drawn ON.
        if self.x in [p - 1, p, p + 1]:
            self.set_char(self.x, self.y, '#')

        # draw to screen
        self.print()

        # increment the to the next pixel to be drawn, incrementing row if necessary
        if self.x == self.width - 1:
            self.x = 0
            self.y += 1
        else:
            self.x += 1

    def print(self):
        os.system("clear")
        for line in self.dots:
            print(line)


def main() -> None:
    console = Console()
    insts = [line.strip() for line in open(FILENAME)]
    signal_strengths = {}
    x = 1
    execute = False

    for i, instr in enumerate(insts):
        cycle = i + 1                           # cycle is 1-indexed
        signal_strengths[cycle] = cycle * x     # this should come BEFORE execution

        # draw X at its current value
        console.draw(x)
        
        if instr.split()[0] == "addx":
            if execute: 
                x += int(instr.split()[1])
                execute = False
                
            else:
                # code works by duplicating the addx instruction to
                # account for it taking two cycles; it only executes
                # the instruction on the second iteration, when
                # execute = true.
                insts.insert(cycle, instr)
                execute = True
    
    print("Part one: ", sum([signal_strengths[20], signal_strengths[60],
        signal_strengths[100], signal_strengths[140],
        signal_strengths[180], signal_strengths[220]]))

if __name__ == "__main__":
    main()
