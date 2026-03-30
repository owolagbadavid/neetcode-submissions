class Solution:

    def encode(self, strs: List[str]) -> str:
        output = ""
        for string in strs:
            for letter in string:
                output += chr(254 if ord(letter) + 1 == 255 else ord(letter) + 1)
            output += chr(255)
        return output
    def decode(self, s: str) -> List[str]:
        output = []
        word = ""
        for l in s:
            if ord(l) == 255:
                output.append(word)
                word = ""
            elif ord(l) == 254:
                word += l
            else:
                word += chr(ord(l) - 1)
        if word:
            output.append(word)
        return output

