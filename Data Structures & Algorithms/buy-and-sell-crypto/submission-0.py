class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        res = 0
        minPrice = prices[0]
        for p in prices:
            minPrice = min(minPrice, p)
            if p > minPrice:
                res = max(res, p - minPrice)
        return res