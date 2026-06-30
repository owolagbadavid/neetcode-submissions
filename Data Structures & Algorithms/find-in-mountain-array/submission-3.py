class Solution:
    def findInMountainArray(self, target: int, mountainArr: 'MountainArray') -> int:
        def search(l, r):
            res = -1
            while l <= r:
                m = (l + r) // 2
                cur = mountainArr.get(m)
                if target <= cur:
                    res = m if target == cur else res
                    r = m-1
                else:
                    l = m + 1
            return res
        
        def searchR(l, r):
            res = -1
            while l <= r:
                m = (l + r) // 2
                cur = mountainArr.get(m)
                if target >= cur:
                    res = m if target == cur else res
                    r = m-1
                else:
                    l = m + 1
            return res

        length = mountainArr.length()
        pivot = (length // 2) - 1 if length % 2 == 0 else (length // 2)
        l, r = 0, length-1
        while l <= r:
            m = (l+r)//2
            left, right, mid = mountainArr.get(m-1), mountainArr.get(m+1), mountainArr.get(m)
            if mid > left and mid > right:
                pivot = m
                break
            elif mid > left and mid < right:
                l = m
            else:
                r = m

        r1 = search(0, pivot)
        r2 = searchR(pivot+1, length-1)

        if r2 == -1:
            return r1
        elif r1 == -1:
            return r2
        else:
            return min(r1, r2)