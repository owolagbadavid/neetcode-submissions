public class Solution {
    public bool IsArraySpecial(int[] nums) {
        var i = 0;
        var j = 1;

        while (j < nums.Length) {
            if (nums[i] % 2 == nums[j] % 2) {
                return false;
            }

            i += 1;
            j += 1;
        }

        return true;
    }
}