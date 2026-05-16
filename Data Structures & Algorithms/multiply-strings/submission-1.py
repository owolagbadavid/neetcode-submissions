class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        def myInt(s: str):
            dic = {
                "1": 1,"0": 0,"2": 2,"3": 3,"4": 4,
                "5": 5,"6": 6,"7": 7,"8": 8,"9": 9
            }
            n = 0
            p = 1
            for i in range(len(s)-1, -1, -1):
                n += dic[s[i]] * p
                p *= 10
            return n

        return str(myInt(num1) * myInt(num2))