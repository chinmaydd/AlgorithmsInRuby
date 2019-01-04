from disjoint import DisjointSet

class GridWorld:
    def __init__(self, n, m):
        self.rows = n
        self.cols = m
        self.grid_sets = DisjointSet()

    # Checks if grid access is valid
    def is_safe(self, i, j):
        return (i >= 0 and i < self.rows) and (j >= 0 and j < self.cols)

    # Add land entity to GridWorld
    def add_land(self, i, j):
        # Create a new set for the land added
        self.grid_sets.make_set((i, j))

        # Join it with adjacent islands
        if self.is_safe(i-1, j) and self.grid_sets.get((i-1, j)):
            self.grid_sets.union(self.grid_sets.get((i, j)), self.grid_sets.get((i-1, j)))

        if self.is_safe(i+1, j) and self.grid_sets.get((i+1, j)):
            self.grid_sets.union(self.grid_sets.get((i, j)), self.grid_sets.get((i+1, j)))

        if self.is_safe(i, j-1) and self.grid_sets.get((i, j-1)):
            self.grid_sets.union(self.grid_sets.get((i, j)), self.grid_sets.get((i, j-1)))

        if self.is_safe(i, j+1) and self.grid_sets.get((i, j+1)):
            self.grid_sets.union(self.grid_sets.get((i, j)), self.grid_sets.get((i, j+1)))

    # Get all islands currently in the GridWorld
    def count_islands(self):
        return self.grid_sets.num_sets()

if __name__ == "__main__":
    w = GridWorld(4, 4)

    w.add_land(0, 0)
    assert w.count_islands() == 1

    w.add_land(0, 2)
    w.add_land(0, 3)
    w.add_land(1, 3)
    assert w.count_islands() == 2

    w.add_land(0, 1)
    assert w.count_islands() == 1

    print "[+] All assertions satisified!"
