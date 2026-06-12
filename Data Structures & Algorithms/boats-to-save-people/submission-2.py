class Solution:
    def numRescueBoats(self, people: List[int], limit: int) -> int:
        people.sort()
        l, r = 0, len(people)-1
        res = 0
        while l <= r:
            if l == r or people[r]+people[l] <= limit:
                l += 1
            r -= 1
            res += 1
        return res