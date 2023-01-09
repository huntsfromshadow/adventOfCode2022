from __future__ import annotations
from typing import List, Optional, Tuple
import re
import math

TOTAL_ROUNDS = 20

class Carload:
    def __init__(self, lines: List[str]):
        self.monkies = []

        grp = []        
        for l in lines:
            if(l != ""):
                grp.append(l)
            else:
                print(grp)
                # Split
                m = Monkey.from_list(grp, self)                
                grp = []
                self.monkies.append(m)

        m = Monkey.from_list(grp, self)
        self.monkies.append(m)

        print(len(self.monkies))
        

    def execute_round(self, round_num: int):
        for m in self.monkies: 
            print(f"{m.full_name()}:")

            for _ in range(len(m.items)):
                m_res = m.run_monkey()
                wp = (m.operation[0], m.operation[1])
                out = f"  Monkey inspects an item with a worry level of {m_res[0]}.\n" + \
                    f"    Worry level is {wp[0]} by {wp[1]} to {m_res[1]}.\n" + \
                    f"    Monkey gets bored with item. Worry level is divided by 3 to {m_res[2]}\n" + \
                    f"    Cur Worry divisible by {m.test} -> {m_res[3]}\n" + \
                    f"    Item with worry level of {m_res[1]} is thrown to monkey {m_res[4]}"
                print(out)

        print("\nRound Result: ", round_num)
        for m in self.monkies:            
            print(m.full_name() + ": " + ','.join(str(x) for x in m.items))


    def throw_item_to(self, item: int, monkey_id: str):

        print(f"Throw item {item} to {monkey_id}")        
        self.monkies[monkey_id].items.append(item)

    def final_result(self):
        r = []
        for m in self.monkies:
            r.append(m.number_inspections)
            print(m.full_name() + f" inspected items {m.number_inspections} times.")
        
        r.sort(reverse=True)
        print("Monkey Buisness: ", r[0] * r[1])



class Monkey:
    def __init__(self, name: str, items: List[int], operation: List[str], \
            test: int, true_opt: int, false_opt: int, carload: Carload):
        self.name = name
        self.items = items
        self.operation = operation
        self.test = test
        self.true_opt = true_opt
        self.false_opt = false_opt
        self.carload = carload
        self.number_inspections = 0

    def __repr__(self) -> str:
        return f"Monkey {self.name}:\n" + \
                f"\Items: {self.items}\n" + \
                f"\tOperation: {self.operation}\n" + \
                f"\tTest: {self.test}\n" + \
                f"\t\tTrue: {self.true_opt}\n" + \
                f"\t\tFalse: {self.false_opt}\n"

    @classmethod
    def from_list(cls, lines: List[str], carload: Carload) -> Monkey:
        name = re.match( r"Monkey (\d+):", lines[0] ).group(1)
        items = list(map(lambda x: int(x.strip()), \
            re.match( r".*Starting items: (.*)", lines[1]).group(1).split(",")))
        operation = re.match( r".*Operation: new = (.*)", lines[2]) \
            .group(1).split(" ")[1:]
        test = int(re.match(r".*Test: divisible by (.*)", lines[3]).group(1))
        true_opt = int(re.match(r".*If true: throw to monkey (.*)", lines[4]).group(1))
        false_opt = int(re.match(r".*If false: throw to monkey (.*)", lines[5]).group(1))

        return cls(name, items, operation, test, true_opt, false_opt, carload)

    def full_name(self) -> str:
        return f"Monkey {self.name}"

    def run_monkey(self) -> Optional[Tuple[int, int, int, bool, int]]:
        if len(self.items) == 0:
            return None
        else:
            starting_worry_value = self.items[0]
            new_worry_level = self.calc_new_worry_level()
            bored_worry_level = math.floor(new_worry_level / 3)

            if bored_worry_level % self.test == 0:
                test_result = True
            else:
                test_result = False

            if test_result:
                throw_to = self.true_opt
            else:
                throw_to = self.false_opt

        # Handle final throw
        self.items = self.items[1:]
        self.carload.throw_item_to(bored_worry_level, throw_to)

        self.number_inspections = self.number_inspections + 1

        return (starting_worry_value, new_worry_level, bored_worry_level, test_result, throw_to)

    def calc_new_worry_level(self) -> int:
        if self.operation[1] == "old":
            sec_val = self.items[0]
        else:
            sec_val = int(self.operation[1])
        
        if self.operation[0] == "*":
            final_val = self.items[0] * sec_val
        elif self.operation[0] == "+":
            final_val = self.items[0] + sec_val
        else:
            raise Exception("Unknown Operation in calc_new_worry_level " + self.operation[0])

        return final_val


def run():
    FILE = "input.txt"
    f = open(FILE, "r")
    lines = f.read().splitlines()

    carload = Carload(lines)

    for i in range(TOTAL_ROUNDS):
        carload.execute_round(i+1)

    carload.final_result()

if __name__ == "__main__":
    run()