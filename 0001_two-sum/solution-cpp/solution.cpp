#include <vector>
#include <unordered_map>

using std::vector;
using std::unordered_map;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        unordered_map<int, int> solutionMap;
        for (int i =0; i < nums.size(); i++) {
            int needed = target - nums[i];
            if (solutionMap.count(needed)) {
                return {solutionMap[needed], i};
            } else {
                solutionMap[nums[i]] = i;
            }
        }
        return {};
    }
};

int main() {
    return 0;
}
