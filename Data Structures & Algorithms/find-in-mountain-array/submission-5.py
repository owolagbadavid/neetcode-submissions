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
        l, r = 1, length-2
        while l <= r:
            m = (l+r)//2
            left, right, mid = mountainArr.get(m-1), mountainArr.get(m+1), mountainArr.get(m)
            if mid > left and mid > right:
                pivot = m
                break
            elif mid > left and mid < right:
                l = m + 1
            else:
                r = m - 1

        r1 = search(0, pivot)
        if r1 == -1:
            return searchR(pivot+1, length-1)
        return r1