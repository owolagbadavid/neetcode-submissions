class CountSquares:

    def __init__(self):
        self.p = defaultdict(int)
        self.smallest = [float('inf'), float('inf')]
        self.largest  = [float('-inf'), float('-inf')]
        self.vec = [[1,-1], [-1,1], [1,1], [-1,-1],]

    def add(self, point: List[int]) -> None:
        self.p[(point[0], point[1])] += 1
        self.largest[0] = max(point[0], self.largest[0])
        self.largest[1] = max(point[1], self.largest[1])
        self.smallest[1] = min(point[1], self.smallest[1])
        self.smallest[0] = min(point[0], self.smallest[0])

    def count(self, point: List[int]) -> int:
        res = 0
        for dx, dy in self.vec:
            nx, ny = point[0]+dx, point[1]+dy
            while (nx >= self.smallest[0] and nx <= self.largest[0]
                and ny >= self.smallest[1] and ny <= self.largest[1]):
                c1 = self.p.get((nx, ny), 0)
                c2 = self.p.get((nx - dx, ny), 0)
                c3 = self.p.get((nx, ny - dy), 0)
                if c1 and c2 and c3:
                    res += c1 * c2 * c3
                dx, dy = dx + math.copysign(1, dx), dy + math.copysign(1, dy)
                nx, ny = point[0]+dx, point[1]+dy

        return res