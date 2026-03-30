class TimeMap:

    def __init__(self):
        self.myMap = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key in self.myMap:
            self.myMap[key].append((timestamp, value))
        else:
            self.myMap[key] = [(timestamp, value)]

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.myMap:
            return ""
        arr = self.myMap[key]

        l , r = 0, len(arr) - 1
        m = (l + r) // 2
        while l <= r:
            m = (l + r) // 2
            if timestamp > arr[m][0]:
                l = m + 1
            elif timestamp < arr[m][0]:
                r = m - 1
            else:
                return arr[m][1]
        if arr[m][0] > timestamp:
            if m <= 0:
                return ""
            else:
                return arr[m - 1][1]
        return arr[m][1]

