class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        carry = 1
        n = len(digits)
        i = n-1
        res = []

        while carry or i >= 0:
            it = 0
            if i >= 0:
                it = digits[i]
            if carry:
                it += carry
                carry = 0
                if it > 9:
                    carry = it // 10
                    it = it % 10
            res.append(it)
            i -= 1
        res.reverse()
        return res