from __future__ import annotations
from typing import Optional, Tuple
import queue

class Node:
    def __init__(self, id: int):
        self.id = id
        self.edges_to = []

    def add_edge_to(self, node: Node):
        self.edges_to.append(node)

    def __repr__(self) -> str:
        out = f"Node: {self.id}\n"

        edgestr = list(map(lambda node: node.id, self.edges_to))
        out = out + f"\tEdges To: {edgestr}\n"
        return out

class Graph:
    def __init__(self):
        self.nodes = []

    def add_node(self, node: Node):
        self.nodes.append(node)

    def get_node(self, node_id: int) -> Optional[Node]:
        for n in self.nodes:
            if n.id == node_id:
                return n
        return None

    def add_edge(self, from_node: int, to_node: int, bidirec=True):
        f_node = self.get_node(from_node)
        t_node = self.get_node(to_node)

        if f_node == None:
            raise Exception("from node not found")

        if t_node == None:
            raise Exception("to node not found")

        f_node.add_edge_to(t_node)
        if bidirec:
            t_node.add_edge_to(f_node)

    def add_edges(self, *argv: Tuple[int, int] ):
        for arg in argv:
            self.add_edge(arg[0], arg[1])

    def __repr__(self) -> str:
        out = ""
        for n in self.nodes:
            out = out + n.__repr__()
        return out + "\n"

    def explore(self):
        start = 1
        end = 7

        node_queue = queue.Queue()
        node_queue.put(1)

        steps = 0
        visited = []
        parent = None

        while True:
            if node_queue.empty():
                break  # We are done

            n = node_queue.get()
            visited.append(n)


            print(f"In {n}")
            if parent == n:
                steps = steps - 1
            else:
                steps = steps + 1

            if n == end:
                break;

            nd = self.get_node(n)
            for e in nd.edges_to:
                if e.id not in visited:
                    node_queue.put(e.id)

            parent = n

        print(steps)







# Test Graph
# [1] - [2]
#  |     |
# [3] - [4] - [8] - [10]
#  |           |     |
# [5] -       [9] ----
#  |   |
# [6] [7]

# Build the Graph
g = Graph()
g.add_node( Node(1) )
g.add_node( Node(2) )
g.add_node( Node(3) )
g.add_node( Node(4) )
g.add_node( Node(5) )
g.add_node( Node(6) )
g.add_node( Node(7) )
g.add_node( Node(8) )
g.add_node( Node(9) )
g.add_node( Node(10) )
g.add_edges( (1, 2), (1,3) )
g.add_edges( (2, 4) )
g.add_edges( (3, 4), (3, 5) )
g.add_edges( (4, 8) )
g.add_edges( (5, 6), (5, 7) )
g.add_edges( (8, 9), (8, 10), (9,10) )

g.explore()

# print(g)
