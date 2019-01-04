# Helper module for GridWorld problem

class Node:
    def __init__(self, rank, val):
        self.rank = rank
        self.parent = self
        self.val = val

class DisjointSet:
    def __init__(self):
        self.members = {}

    def get(self, val):
        if val in self.members:
            return self.members[val]
        else:
            return None

    def make_set(self, val):
        if val not in self.members:
            self.members[val] = Node(0, val)

    def find(self, node):
        if node.parent != node:
            self.members[node.val].parent = self.find(node.parent)

        return node.parent

    def union(self, node_1, node_2):
        root_1 = self.find(node_1)
        root_2 = self.find(node_2)

        if root_1 == root_2:
            return

        if root_1.rank < root_2.rank:
            root_1, root_2 = root_2, root_1

        self.members[root_2.val].parent = root_1
        if root_1.rank == root_2.rank:
            # Increase rank if new parent has same rank as child
            self.members[root_1.val].rank = self.members[root_1.val].rank + 1

    def num_sets(self):
        count = 0
        for member in self.members:
            if self.members[member].parent == self.members[member]:
                count += 1

        return count
                
if __name__ == "__main__":
    d = DisjointSet()

    for i in range(10):
        d.make_set(i)
    
    d.union(d.get(2), d.get(4))
    d.union(d.get(2), d.get(1))
