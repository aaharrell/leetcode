public class Solution {
    public string LongestCommonPrefix(string[] strs) {
        string prefix = "";
        if (strs.Length == 0) {
            return prefix;
        }

        for (int i = 0; i < strs[0].Length; i++) {
            char currentLetter = strs[0][i];
            for (int j = 0; j < strs.Length; j++) {
                if ((strs[j].Length <= i) || (strs[j][i] != currentLetter)) {
                    return prefix;
                }
            }
            prefix += currentLetter;
        }
        return prefix.ToString();
    }
}

public class Program {
    public static void Main(string[] args) {
        Solution prefix = new Solution();
        string[] words1 = {"flower", "flow", "flight"};
        string result1 = prefix.LongestCommonPrefix(words1);
        Console.WriteLine(result1);

        string[] words2 = {"dog", "racecar", "car"};
        string result2 = prefix.LongestCommonPrefix(words2);
        Console.WriteLine(result2);
    }
}
