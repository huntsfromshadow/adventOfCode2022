from typing import List
import re
from math import floor

FILENAME = "input.txt"
ALL_MONKEYS: List["Monkey"] = []

# THIS. THIS is what took me so long to understand EVEN AFTER
# looking at other people's answers. 
# Basically if ALL the items are divided by the SAME number,
# (not an arbitrary number, but the product of all denominators),
# then it won't change what each individual item's result will
# be when divided by its monkey's specific denominator. 
# ...
# I think.
MOD_CONSTANT = 1        

class Monkey():
    def __init__(self, raw_str: str) -> None:
        global MOD_CONSTANT

        self.inspected_items = 0

        lines = [line.strip() for line in raw_str.split("\n")]

        # index in ALL_MONKEYS
        self.index = int(re.findall('[\d]', lines[0])[0])

        # starting items
        self.items: List[int] = []
        for item in lines[1].split(":")[1:]:
            nums = [int(num.strip()) for num in item.split(',')]
            self.items += nums

        # operation to perform on each item
        line = lines[2].split(": ")[1]
        self.new_line = line.replace("new = ", "lambda old: ")
        self.func = eval(self.new_line)

        # test; here i hard-code a bit: all tests are division
        self.denominator = int(lines[3].split(': ')[1].split()[-1])
        MOD_CONSTANT *= self.denominator
        self.test = lambda x: x % self.denominator == 0
    
        # true/false branches
        self.true = int(lines[4].split()[-1])
        self.false: int = int(lines[5].split()[-1])
    
    def inspect(self):
        if len(self.items) >= 1:
            item = self.items[0]
            for item in self.items:
                self.inspected_items += 1

                # part one
                # new = floor(self.func(item) / 3)

                # part two
                new = (self.func(item)) % MOD_CONSTANT

                if self.test(new): 
                    ALL_MONKEYS[self.true].items.append(new)
                else: 
                    ALL_MONKEYS[self.false].items.append(new)
            
            self.items = []
        
        try:
            return ALL_MONKEYS[self.index + 1].inspect()
        except:
            return None


def main():
    input = open(FILENAME).read().split("\n\n")
    for group in input:
        ALL_MONKEYS.append(Monkey(group))

    for i in range(10000):
        ALL_MONKEYS[0].inspect()

    inspected_totals = sorted([monkey.inspected_items for monkey in ALL_MONKEYS])

    # part one
    # print((inspected_totals[-2] * 3) * (inspected_totals[-1] * 3))

    # part two
    print(inspected_totals[-2] * inspected_totals[-1])


if __name__ == "__main__":
    main()